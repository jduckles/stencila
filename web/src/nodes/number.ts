import { html, LitElement } from 'lit'
import { customElement } from 'lit/decorators.js'

import { withTwind } from '../twind'
import { closestGlobally } from '../utilities/closestGlobally'

import '../ui/nodes/cards/inline-on-demand'

/**
 * Web component representing a Stencila Schema `Number` node
 *
 * @see https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/number.md
 */
@customElement('stencila-number')
@withTwind()
export class Number extends LitElement {
  override render() {
    return closestGlobally(this, 'stencila-code-expression')
      ? html`<slot></slot>`
      : html`<stencila-ui-inline-on-demand type="Number">
          <span slot="content"><slot></slot></span>
        </stencila-ui-inline-on-demand>`
  }
}
