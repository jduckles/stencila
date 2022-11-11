import { sentenceCase } from 'change-case'
import {
  wrappingInputRule,
  textblockTypeInputRule,
  emDash,
  ellipsis,
  InputRule,
} from 'prosemirror-inputrules'
import { MarkType, NodeRange, NodeType } from 'prosemirror-model'
import { Plugin, TextSelection, Transaction } from 'prosemirror-state'
import { EditorView } from 'prosemirror-view'

import { articleSchema } from './nodes'
const { marks, nodes } = articleSchema

/**
 * The ProseMirror `InputRules` (i.e. input macros) for a Stencila `Article`
 *
 * Order is important: the first matching rule is used. So generally, rules
 * for blocks should go first, then inlines, then marks, and finally text transforms.
 * Other than that, rules are sorted alphabetically below mainly to make
 * it easier to keep track of which nodes types have input rules.
 *
 * For docs and examples see:
 *  - https://prosemirror.net/docs/ref/#inputrules
 *  - https://github.com/ProseMirror/prosemirror-example-setup/blob/master/src/inputrules.js
 */
export const articleInputRules = inputRules({
  rules: [
    ////////////////////////////////////////////////////////////////
    // Block nodes
    //
    // For consistency all of these are only triggered on enter
    // (only for some of these is it possible to not have enter at end)
    ////////////////////////////////////////////////////////////////

    // Call
    blockInputRule(/^\/([^\(]+)\(([^\)]*)\)\n$/, nodes.Call, (match) => ({
      source: match[1],
      // TODO: parse args
    })),

    // CodeBlock
    blockInputRule(/^```(\w+)?\n$/, nodes.CodeBlock, (match) => ({
      programmingLanguage: match[1] ?? undefined,
    })),

    // CodeChunk
    blockInputRule(
      /^```(\w+)?(?:(?:\s*!)|(?:\s+exec))\n$/,
      nodes.CodeChunk,
      (match) => ({
        programmingLanguage: match[1] ?? undefined,
      })
    ),

    // For
    blockInputRule(
      /^(?::{3,})?\s*for\s+(\w+)\s+in\s+([^{]+)({\w+})?\n$/,
      nodes.For,
      (match) => ({
        symbol: match[1],
        text: match[2],
        programmingLanguage: match[3],
      })
    ),

    // Form
    blockInputRule(
      /^(?::{3,})?\s*form(?:\s+(?:to\s+)?(create|update|delete|(?:update or delete))\s+(\w+))?\n$/,
      nodes.Form,
      (match) => {
        const deriveAction = ((action) => {
          switch (action) {
            case 'update or delete':
              return 'UpdateOrDelete'
            default:
              return sentenceCase(action)
          }
        })(match[1])

        return {
          deriveAction,
          deriveFrom: match[2],
        }
      }
    ),

    // Heading
    textblockTypeInputRule(/^(#{1,6})\s$/, nodes.Heading, (match) => ({
      depth: match?.[1]?.length,
    })),

    // If
    new InputRule(
      /^(?::{3,})?\s*if\s([^{]+)({\w+})?\n$/,
      (state, match, start, end) => {
        return state.tr.replaceWith(
          start - 1,
          end + 1,
          nodes.If.create({}, [
            nodes.IfClause.create({
              text: match[1],
              programmingLanguage: match[2],
            }),
          ])
        )
      }
    ),

    // Division
    //
    // Needs to come after other rules that start with ::: e.g. `If`.
    // Does not allow for style expressions in different languages because
    // these usually need to be enclosed by quotes which are consumed by the
    // input rule for `Quote` and are thus "not available" by the time we
    // get to the end of the line.
    blockInputRule(
      /^(?:(?::{3,})|(?:div\s))\s*([^\n]*)\n$/,
      nodes.Division,
      (match) => ({
        text: match[1],
      })
    ),

    // Include
    blockInputRule(
      /^(?:\/|(?:include\s+))([^ ]+)(?:\s+select\s+([^\n]+))?\n$/,
      nodes.Include,
      (match) => ({
        source: match[1],
        select: match[2],
      })
    ),

    // List
    wrappingInputRule(/^\s*([-+*])\s$/, nodes.List, {
      order: 'Unordered',
    }),
    wrappingInputRule(/^(\d+)\.\s$/, nodes.List, {
      order: 'Ascending',
    }),

    // MathBlock
    blockInputRule(/^\$\$\n/, nodes.MathBlock, (match) => ({
      text: match[1],
      mathLanguage: 'tex',
    })),

    // QuoteBlock
    wrappingInputRule(/^\s*>\s$/, nodes.QuoteBlock),

    // Table
    // Markdown style pipe table header
    new InputRule(/^\|(\s*[^|]*\|)+\n$/, (state, match, start, end) => {
      const cols = match[0]
        .trim()
        .split('|')
        .filter((col) => col.length > 0)
      return state.tr.replaceWith(
        start - 1,
        end + 1,
        nodes.Table.create(
          {},
          cols.map((col: string) =>
            nodes.TableHeader.create(null, articleSchema.text(col.trim()))
          )
        )
      )
    }),

    // ThematicBreak (three or more asterisks or underscores; not dashes because they are consumed by emdash)
    blockInputRule(/^(\*{3,})|(_{3,})$/, nodes.ThematicBreak),

    ////////////////////////////////////////////////////////////////
    // Inline nodes
    ////////////////////////////////////////////////////////////////

    // Button
    inlineInputRule(/#\[([^\]]+)\]$/, nodes.Button, (match) => ({
      name: match[1],
    })),

    // CodeExpression
    inlineInputRule(
      /(^|[^`])`([^`]*)`(?:(?:{([a-z]+) exec})|(?:([a-z]+)!))$/,
      nodes.CodeExpression,
      (match) => ({
        text: match[2],
        programmingLanguage: match[3] ?? match[4],
      }),
      (match) => match[1].length
    ),

    // CodeFragment
    inlineInputRule(
      /(^|[^`])`([^`]*)`(?:(?:{([a-z]+)})|(?:([a-z]+)\s)|\s)$/,
      nodes.CodeFragment,
      (match) => ({
        text: match[2],
        programmingLanguage: match[3] ?? match[4],
      }),
      (match) => match[1].length
    ),

    // MathFragment
    inlineInputRule(
      /(^|[^\$])\$([^\$]+)\$$/,
      nodes.MathFragment,
      (match) => ({
        text: match[2],
        mathLanguage: 'tex',
      }),
      (match) => match[1].length
    ),

    // Parameter
    inlineInputRule(
      /&\[([^\]]+)\](?:(?:{([a-z]+)})|(?:([a-z]+)\s)|\s)$/,
      nodes.Parameter,
      (match) => {
        const type = ((type: string) => {
          switch (type) {
            case 'enum':
              return 'enum'
            case 'bool':
            case 'boolean':
              return 'boolean'
            case 'int':
            case 'integer':
              return 'integer'
            case 'num':
            case 'real':
              return 'number'
            case 'str':
            case 'string':
              return 'string'
            case 'date':
              return 'date'
            case 'time':
              return 'time'
            case 'datetime':
              return 'datetime'
            case 'timestamp':
              return 'timestamp'
            case 'duration':
              return 'duration'
            default:
              return undefined
          }
        })(match[2] ?? match[3])

        const validator = type
          ? `<stencila-${type}-validator></stencila-${type}-validator>`
          : '<stencila-validator></stencila-validator>'

        return {
          name: match[1],
          validator,
        }
      }
    ),

    // Span
    new InputRule(
      //  content    style
      /\[([^\]]+)\]`([^`]+)`$/,
      (state, match, start, end) => {
        const tr = state.tr

        // Delete the square brackets and backticked style
        tr.delete(end - match[2].length - 2, end)
        tr.delete(start, start + 1)

        // Wrap the remaining content in a span
        const startPos = tr.doc.resolve(start)
        const endPos = tr.doc.resolve(end - 1 - match[2].length - 2)
        const range = new NodeRange(startPos, endPos, 2)
        tr.wrap(range, [
          {
            type: nodes.Span,
            attrs: {
              text: match[2],
            },
          },
        ])
        return tr
      }
    ),

    ////////////////////////////////////////////////////////////////
    // Marks
    ////////////////////////////////////////////////////////////////

    // Emphasis (delimited by single asterisk or underscore; no spaces)
    markInputRule(
      /(?:^|[^\*])(?:\*)([^\*\s]+)(?:\*)$/,
      marks.Emphasis,
      undefined,
      /^[^\*]/
    ),
    markInputRule(
      /(?:^|[^_])(?:_)([^_\s]+)(?:_)$/,
      marks.Emphasis,
      undefined,
      /^[^_]/
    ),

    // Quote (delimited by single or double quotes)
    markInputRule(/(?:"|“)([^"”]+)(?:"|”)$/, marks.Quote),
    markInputRule(/(?:'|‘)([^'|’]+)(?:'|’)$/, marks.Quote),

    // Strikeout (delimited by two tildes)
    markInputRule(/(?:~~)([^~]+)(?:~~)$/, marks.Strikeout),

    // Strong (delimited by two asterisks; no spaces)
    markInputRule(/(?:\*\*)([^\*\s]+)(?:\*\*)$/, marks.Strong),

    // Subscript (delimited by single tilde; no spaces)
    markInputRule(
      /(?:^|[^~])(?:~)([^~\s]+)(?:~)$/,
      marks.Subscript,
      undefined,
      /^[^~]/
    ),

    // Superscript (delimited by single caret; no spaces)
    markInputRule(/(?:\^)([^\^\s]+)(?:\^)$/, marks.Superscript),

    // Underline (delimited by two underscores)
    markInputRule(/(?:__)([^_\s]+)(?:__)$/, marks.Underline),

    ////////////////////////////////////////////////////////////////
    // Text context
    ////////////////////////////////////////////////////////////////

    // Converts double dashes to an emdash
    emDash,

    // Converts three dots to an ellipsis character
    ellipsis,
  ],
})

/**
 * A patched version of `prosemirror-inputrules.inputRules` function
 * that runs rules on `Enter`
 *
 * Necessary for input rules for code blocks, code chunks, divs etc where the user
 * shouldn't have to (or can't because of regex) enter a space at end of paragraph
 * to trigger a rule.
 *
 * Credit: https://discuss.prosemirror.net/t/trigger-inputrule-on-enter/1118/5
 * Original source: https://github.com/ProseMirror/prosemirror-inputrules/blob/d0e2fdf18df2bce780877705aa198675ac1d1799/src/inputrules.ts#L59
 */
export function inputRules({ rules }: { rules: readonly InputRule[] }) {
  let plugin: Plugin<{
    transform: Transaction
    from: number
    to: number
    text: string
  } | null> = new Plugin({
    state: {
      init() {
        return null
      },
      apply(this: typeof plugin, tr, prev) {
        let stored = tr.getMeta(this)
        if (stored) return stored
        return tr.selectionSet || tr.docChanged ? null : prev
      },
    },

    props: {
      handleTextInput(view, from, to, text) {
        return run(view, from, to, text, rules, plugin)
      },
      handleDOMEvents: {
        compositionend: (view) => {
          setTimeout(() => {
            let { $cursor } = view.state.selection as TextSelection
            if ($cursor) run(view, $cursor.pos, $cursor.pos, '', rules, plugin)
          })
        },
      },
      // Inserted code...
      handleKeyDown(view, event) {
        if (event.key !== 'Enter') return false
        let { $cursor } = view.state.selection as TextSelection
        if ($cursor)
          return run(view, $cursor.pos, $cursor.pos, '\n', rules, plugin)
        return false
      },
    },

    isInputRules: true,
  })
  return plugin

  function run(
    view: EditorView,
    from: number,
    to: number,
    text: string,
    rules: readonly InputRule[],
    plugin: Plugin
  ) {
    const MAX_MATCH = 500
    if (view.composing) return false
    let state = view.state,
      $from = state.doc.resolve(from)
    if ($from.parent.type.spec.code) return false
    let textBefore =
      $from.parent.textBetween(
        Math.max(0, $from.parentOffset - MAX_MATCH),
        $from.parentOffset,
        null,
        '\ufffc'
      ) + text
    for (let i = 0; i < rules.length; i++) {
      // @ts-expect-error because match is @internal
      let match = rules[i].match.exec(textBefore)
      let tr =
        match &&
        // @ts-expect-error because handler is @internal
        rules[i].handler(
          state,
          match,
          from - (match[0].length - text.length),
          to
        )
      if (!tr) continue
      view.dispatch(tr.setMeta(plugin, { transform: tr, from, to, text }))
      return true
    }
    return false
  }
}

/**
 * Create an input rule to create a block node
 */
function blockInputRule(
  regexp: RegExp,
  nodeType: NodeType,
  getAttrs?: (matches: RegExpMatchArray) => Record<string, string>
): InputRule {
  return new InputRule(regexp, (state, match, start, end) => {
    return state.tr.replaceWith(
      start - 1,
      end + 1,
      nodeType.create(getAttrs?.(match))
    )
  })
}

/**
 * Create an input rule to create an inline node
 */
function inlineInputRule(
  regexp: RegExp,
  nodeType: NodeType,
  getAttrs?: (matches: RegExpMatchArray) => Record<string, string>,
  offsetStart?: (matches: RegExpMatchArray) => number
): InputRule {
  return new InputRule(regexp, (state, match, start, end) => {
    return state.tr.replaceWith(
      start + (offsetStart?.(match) ?? 0),
      end,
      nodeType.create(getAttrs?.(match))
    )
  })
}

/**
 * Create an input rule to place mark around the matched text.
 *
 * Credits: https://discuss.prosemirror.net/t/input-rules-for-wrapping-marks/537/12
 */
function markInputRule(
  regexp: RegExp,
  markType: MarkType,
  getAttrs?: (
    matches: RegExpMatchArray
  ) => Record<string, string> | Record<string, string>,
  skipStart?: RegExp
): InputRule {
  return new InputRule(regexp, (state, match, start, end) => {
    const attrs = (
      getAttrs instanceof Function ? getAttrs(match) : getAttrs
    ) as Record<string, string>

    const tr = state.tr
    if (match[1]) {
      let skipMatch
      let skipLen = 0

      if ((skipMatch = skipStart && match[0].match(skipStart))) {
        skipLen = skipMatch[0].length
        start += skipLen
      }

      let textStart = start + match[0].indexOf(match[1]) - skipLen
      let textEnd = textStart + match[1].length
      if (textEnd < end) tr.delete(textEnd, end)
      if (textStart > start) tr.delete(start, textStart)
      end = start + match[1].length
    }
    tr.addMark(start, end, markType.create(attrs))
    tr.removeStoredMark(markType) // Do not continue with mark.

    return tr
  })
}
