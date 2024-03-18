import { html } from 'lit'
import { customElement } from 'lit/decorators.js'

import '../ui/nodes/card'

import { Entity } from './entity'

/**
 * Web component representing a Stencila Schema `Integer` node
 *
 * Note that this extends `Entity`, despite not doing so in Stencila Schema`, to
 * make use of the various `render*View()` methods.
 *
 * @see https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/integer.md
 */
@customElement('stencila-integer')
export class Integer extends Entity {
  /**
   * In static view just render the value
   */
  override renderStaticView() {
    return html`<slot></slot>`
  }

  /**
   * In dynamic view, in addition to the value, render a node card.
   */
  override renderDynamicView() {
    return html`<stencila-ui-node-card
      type="Integer"
      view="dynamic"
      ?collapsible=${true}
      ><div slot="body"><slot></slot></div
    ></stencila-ui-node-card>`
  }

  /**
   * In source view, render the same as dynamic view, including the
   * value since that won't be a present in the source usually (given this
   * node type is normally only present in `CodeChunk.outputs` and `CodeExpression.output`).
   */
  override renderSourceView() {
    return html`<stencila-ui-node-card
      type="Integer"
      view="source"
      ?collapsible=${true}
      ><div slot="body"><slot></slot></div
    ></stencila-ui-node-card>`
  }
}
