import { Idiomorph } from 'idiomorph/dist/idiomorph.esm.js'

import { type DocumentId } from '../types'

import { FormatClient } from './format'

/**
 * A read-only client that keeps a DOM element synchronized with the HTML
 * representation of a document on the server
 *
 * This class simply extends `FormatClient` and uses morphing
 * to update the DOM element whenever the HTML changes.
 */
export class DomClient extends FormatClient {
  /**
   * Construct a new `DomClient`
   *
   * @param id The id of the document
   * @param elem The DOM element that will be updated
   */
  constructor(id: DocumentId, elem: HTMLElement) {
    super(id, 'read', 'dom')

    this.subscribe((html) => {
      if (process.env.NODE_ENV === 'development') {
        console.log(`📝 DomClient morphing element`, elem)
      }

      // Update element
      // Any errors during morphing (i.e is somehow the HTML is invalid)
      // result in a reset request being sent to the server
      try {
        Idiomorph.morph(elem, html)
      } catch (error) {
        console.log('While morphing DOM', error)
        this.sendMessage({ version: 0 })
      }
    })
  }
}
