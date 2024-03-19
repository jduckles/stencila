import { html } from 'lit'
import { customElement } from 'lit/decorators'

import { withTwind } from '../twind'

import { Entity } from './entity'

@customElement('stencila-code-inline')
@withTwind()
export class CodeInline extends Entity {
  override renderStaticView() {
    return html`<slot name="code"></slot>`
  }

  override renderDynamicView() {
    return this.renderStaticView()
  }

  override renderVisualView() {
    return this.renderStaticView()
  }

  override renderSourceView() {
    return html`
      <stencila-ui-node-card type="CodeInline" view="source">
        <div slot="body">
          <stencila-ui-node-authors>
            <slot name="authors"></slot>
          </stencila-ui-node-authors>
        </div>
      </stencila-ui-node-card>
    `
  }
}