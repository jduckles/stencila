import { html } from 'lit'
import { customElement } from 'lit/decorators'

import { withTwind } from '../twind'

import { Math } from './math'

@customElement('stencila-math-block')
@withTwind()
export class MathBlock extends Math {
  override renderStaticView() {
    return html`<code>${this.code}</code>`
  }

  override renderDynamicView() {
    return this.renderStaticView()
  }

  override renderVisualView() {
    return this.renderStaticView()
  }

  override renderSourceView() {
    return html`
      <stencila-ui-node-card type="MathBlock" view="source">
        <div slot="body">
          <stencila-ui-node-authors>
            <slot name="authors"></slot>
          </stencila-ui-node-authors>
        </div>
      </stencila-ui-node-card>
    `
  }
}