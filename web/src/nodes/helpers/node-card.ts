import '@shoelace-style/shoelace/dist/components/icon/icon'
import type { NodeType } from '@stencila/types'
import { apply } from '@twind/core'
import { html, LitElement } from 'lit'
import { customElement, property } from 'lit/decorators'

import { withTwind } from '../../twind'
import { DocumentView } from '../../types'

import { nodeUi } from './node-ui'

/**
 * A component for displaying information about a node type (e.g. a `Heading` or `Table`)
 */
@customElement('stencila-node-card')
@withTwind()
export class NodeCard extends LitElement {
  /**
   * The type of node that this card is for
   *
   * Used to determine the title, icon and colors of the card.
   */
  @property()
  type: NodeType

  override render() {
    const { iconLibrary, icon, title, colour, borderColour } = nodeUi(this.type)

    const headerStyles = apply([
      'w-full',
      'p-4',
      `bg-[${borderColour}]`,
      `border border-[${borderColour}] rounded-t`,
      'font-medium',
    ])

    const bodyStyles = apply([
      'w-full',
      'p-4',
      `bg-[${colour}]`,
      `border border-[${borderColour}] rounded-b`,
    ])

    return html`
      <div class=${headerStyles}>
        <span class="items-center font-medium flex" style="font-bold">
          <sl-icon library=${iconLibrary} name=${icon} class="pr-2"></sl-icon>
          ${title}
        </span>
      </div>
      <div class=${bodyStyles}>
        <slot></slot>
      </div>
    `
  }
}

export const nodeCardParentStyles = (view: DocumentView) =>
  view !== 'source' ? 'group relative' : ''

export const nodeCardStyles = (view: DocumentView) =>
  view !== 'source'
    ? 'hidden absolute z-10 top-full right-0 group-hover:block'
    : ''