import { createContext } from '@lit/context'

import { DocumentView } from '../types'

export type SidebarContext = {
  /**
   * Whether the directory tree view is open
   */
  directoryOpen: boolean

  /**
   * The current document view
   *
   * When a document is opened, it will be opened with this view.
   */
  currentView?: DocumentView
}

export const sidebarContext = createContext<SidebarContext>('sidebar')
