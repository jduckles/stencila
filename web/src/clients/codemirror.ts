import { EditorView, ViewUpdate } from "@codemirror/view";
import { Extension, TransactionSpec } from "@codemirror/state";

import { StringClient, StringOp, StringPatch } from "./string";

/// The number milliseconds to debounce sending updates
const SEND_DEBOUNCE = 300;

/**
 * A client that keeps a CodeMirror editor synchronized with a buffer
 * on the server
 *
 * This client is read-write: it can send and receives patches to and from
 * the server. To send patches it is necessary to create the client first
 * and add the return value of `sendPatches` to the `extensions of the editor
 * e.g.
 *
 * ```ts
 * const client = CodeMirrorClient("markdown");
 *
 * const editor = new EditorView({
 *   extensions: [client.sendPatches()]
 *   ..
 * })
 *
 * client.receivePatches(editor)
 * ```
 */
export class CodeMirrorClient extends StringClient {
  /**
   * The CodeMirror view to update with patches from the server
   */
  editor?: EditorView;

  /**
   * Whether updates from the editor should be ignored
   *
   * Used to temporarily ignore updates while applying patches from
   * the server.
   */
  ignoreUpdates = false;

  /**
   * A cache of `StringOp`s used to debounce sending patches to the server
   */
  cachedOperations: StringOp[] = [];

  /**
   * Construct a new `CodeMirrorClient`
   *
   * @param format The format of the editor content (e.g. "markdown")
   */
  constructor(format: string) {
    super(format);
  }

  /**
   * Send patches to the server by listening to updates from the code editor
   *
   * @returns A CodeMirror `Extension` to use when creating a new editor
   */
  sendPatches(): Extension {
    let timer: string | number | NodeJS.Timeout;
    return EditorView.updateListener.of((update: ViewUpdate) => {
      if (this.ignoreUpdates || !update.docChanged) {
        return;
      }

      update.changes.iterChanges((from, to, fromB, toB, inserted) => {
        const insert = inserted.toJSON().join("\n");
        this.cachedOperations.push({ from, to, insert });
      });

      clearTimeout(timer);

      timer = setTimeout(() => {
        // If the last operation is only inserting whitespace, do not send.
        // This needs to be more refined: it needs to allow for spaces to be
        // inserted in paragraphs and send immediately, but not spaces at end of
        // paragraphs.
        const op = this.cachedOperations[this.cachedOperations.length - 1];
        if (op.insert.length > 0 && op.insert.trim().length === 0) {
          return;
        }

        // Send the patch
        this.sendMessage({
          version: this.version,
          ops: this.cachedOperations,
        });

        // Increment version and clear cache of ops
        this.version += 1;
        this.cachedOperations = [];
      }, SEND_DEBOUNCE);
    });
  }

  /**
   * Receive patches from the server and apply them to the content of the code editor
   *
   * @param editor The editor that will receive patches from the server
   */
  receivePatches(editor: EditorView) {
    this.editor = editor;

    // Set the initial content of the code editor to the current state
    editor.dispatch({
      changes: { from: 0, to: editor.state.doc.length, insert: this.state },
    });
  }

  // Override `onmessage` to forward patches directly to the code editor
  // instead of updating local string
  receiveMessage(message: Record<string, unknown>) {
    const { version, ops } = message as unknown as StringPatch;

    // Is the patch a reset patch?
    const isReset = ops.length === 1 && ops[0].from === 0 && ops[0].to === 0;

    // Check for non-sequential patch and request a reset patch if necessary
    if (!isReset && version != this.version + 1) {
      this.sendMessage({ version: 0 });
      return;
    }

    // Create a transaction for the patch
    let transaction: TransactionSpec;
    if (isReset) {
      transaction = this.editor.state.update({
        changes: {
          from: 0,
          to: this.editor.state.doc.length,
          insert: ops[0].insert,
        },
        selection: this.editor.state.selection,
      });
    } else {
      transaction = { changes: ops };
    }

    // Dispatch the transaction, ignoring any updates while doing so
    this.ignoreUpdates = true;
    this.editor.dispatch(transaction);
    this.ignoreUpdates = false;

    // Update local version number
    this.version = version;
  }
}
