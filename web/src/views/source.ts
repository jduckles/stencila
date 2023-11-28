import { EditorView as CodeMirrorView } from "@codemirror/view";
import { LitElement } from "lit";
import { customElement, property } from "lit/decorators.js";

import { Capability } from "../capability";
import { CodeMirrorClient } from "../clients/codemirror";

import "./source.css";

/**
 * Source code editor for a document
 *
 * A view which provides read-write access to the document using
 * a particular format.
 */
@customElement("stencila-source")
export class Source extends LitElement {
  /**
   * The capability of the editor
   *
   * This property is passed through to the `CodeMirrorClient`
   * and used to determine whether or not the document is
   * read-only or writable.
   * 
   * This should not be `edit`, `write` or `admin` since this view
   * does not provide the means to modify those.
   */
  @property()
  capability: Capability = "write";

  /**
   * The format of the source code
   */
  @property()
  format: string = "markdown";

  /**
   * A read-write client which sends and receives string patches
   * for the source code to and from the server
   */
  private codeMirrorClient: CodeMirrorClient;

  /**
   * A CodeMirror editor view which the client interacts with
   */
  private codeMirrorView: CodeMirrorView;

  /**
   * Override so that the `CodeMirrorView` is instantiated _after_ this
   * element has a `renderRoot`.
   *
   * @override
   */
  connectedCallback() {
    super.connectedCallback();

    this.codeMirrorClient = new CodeMirrorClient(this.id, this.capability, this.format);

    this.codeMirrorView = new CodeMirrorView({
      extensions: [this.codeMirrorClient.sendPatches()],
      parent: this.renderRoot,
    });

    this.codeMirrorClient.receivePatches(this.codeMirrorView);
  }
}