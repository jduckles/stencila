import SlMenuItem from '@shoelace-style/shoelace/dist/components/menu-item/menu-item.component.js'
import { apply, css } from '@twind/core'
import { html } from 'lit'
import { customElement, property } from 'lit/decorators.js'

import type { DocumentView } from '../../types'
import { TWLitElement } from '../twind'

/**
 * Enhance the event type to include shoelace's event details.
 */
export type UISelectorSelectedEvent = Event & { detail: { item: SlMenuItem } }

/**
 * UI selector
 *
 * A selector that updates some display portion of the UI
 */
@customElement('stencila-ui-selector')
export class UISelector extends TWLitElement {
  /**
   * Label displayed when listbox is not open
   */
  @property()
  label: string = ''

  /**
   * List of values to render in the list
   */
  @property({ type: Array })
  list: [string, string][] = []

  /**
   * Event to call when a list element is selected
   */
  @property()
  clickEvent: (e: UISelectorSelectedEvent['detail']) => void | undefined

  /**
   * Target property in parent component to evaluate
   */
  @property()
  target: DocumentView | string

  /**
   * css identifier to allow querying of the element in order to add event
   * handlers. THIS MUST BE UNIQUE.
   */
  @property()
  targetClass: string | undefined

  override render() {
    const styles = apply([
      'group',
      'text-gray-aluminium',
      'p-0',
      'relative',
      'block',
      'flex-grow',
      'open:text-brand-blue',
      'open:border-b-brand-blue',
    ])

    const classes = css`
      &[open] ::part(caret) {
        transform: rotate(180deg);
      }
    `

    return html`<sl-dropdown
      class="${this.targetClass ?? ''} ${classes} ${styles}"
    >
      ${this.renderButton()} ${this.renderList()}
    </sl-dropdown>`
  }

  private renderButton() {
    const styles = apply([
      'text-base font-bold leading-none',
      'appearance-none select-none',
      'min-w-fit',
      'py-0 px-4',
      'bg-white',
      'border-b-4 border-b-transparent',
      'transition-all ease-in-out',
      'flex',
      'items-center',
      'group-hover:text-brand-blue group-hover:border-b-brand-blue',
    ])

    const classes = css`
      &::part(base) {
        border: none;
        padding: 0;
        outline: none;

        &:hover {
          background: none;
        }
      }

      &::part(label) {
        padding-left: 0;
      }
    `

    return html`<sl-button slot="trigger" class="${styles} ${classes}" caret
      >${this.label}</sl-button
    >`
  }

  private renderList() {
    const styles = apply([
      'block',
      'rounded-b-md border-t-4 border-t-brand-blue',
      'shadow-[0_8px_8px_hsla(0,0%,4%,.1)]',
      'flex flex-col',
      'bg-white',
      '-mt-1',
    ])

    return html`<sl-menu class="${styles}">
      ${this.list.map(([value, label]) => this.renderListItem(value, label))}
    </sl-menu>`
  }

  private renderListItem(value: string, label: string) {
    const styles = apply([
      'block',
      'w-full',
      'py-2 pl-4 pr-12',
      'bg-white',
      'text-sm font-bold text-gray-aluminium',
      'text-left',
      'border-l-4',
      this.target === value ? 'border-brand-blue' : 'border-transparent',
      'hover:bg-gray-wild-sand hover:text-black',
    ])

    const classes = css`
      &::part(checked-icon),
      &::part(submenu-icon) {
        display: none;
      }

      &::part(base) {
        padding: 0;
      }
    `

    return html`<sl-menu-item value="${value}" class="${styles} ${classes}"
      >${label}</sl-menu-item
    >`
  }

  override firstUpdated() {
    const menu = this.renderRoot.querySelector(`.${this.targetClass}`)

    menu.addEventListener(
      'sl-select',
      ({ detail }: Event & { detail: { item: SlMenuItem } }) => {
        this.clickEvent && this.clickEvent(detail)
      }
    )
  }
}
