import { Extension } from '@codemirror/state'
import { ViewPlugin, EditorView, ViewUpdate } from '@codemirror/view'

import { SourceView } from '../source'

/**
 * Returns a CodeMirror `Extension` which creates a new sidebar Element
 * which should be inserted to the right of the CodeMirror content,
 *
 * This Bar should show the info box of the node where the cursor is present
 *
 * @param sourceView The `SourceView` that this extension is for
 * @returns An `Extension` for codemirror
 */
const infoSideBarExtension = (sourceView: SourceView): Extension => {
  const infoSideBarPlugin = ViewPlugin.fromClass(
    class {
      /**
       * The side bar container element
       */
      dom: HTMLElement

      /**
       * The current info box element displayed
       */
      currentInfoBox: HTMLElement = null

      /**
       * Id of the currently cloned node for the info box
       */
      currentNodeId: string

      /**
       * the Y coordinate of the cursor (relative to the viewport)
       */
      cursorY: number

      /**
       * twind classlist for the sidebar dom element
       */
      private domClassList = [
        'absolute',
        'top-0',
        'right-4',
        'h-full',
        'w-full',
        'max-w-[25%]',
        'pb-6', // offset bottom panel
      ]

      constructor(readonly view: EditorView) {
        this.dom = document.createElement('div')

        // this class has no functionality at this point
        // but may be needed for selecting

        this.dom.classList.add('cm-info-sidebar', ...this.domClassList)
        this.view.dom.appendChild(this.dom)
      }

      update = (update: ViewUpdate) => {
        const { view: currentView } = update

        // update height of dom
        this.dom.style.minHeight = `${
          currentView.contentHeight / currentView.scaleY
        }px`

        const cursor = currentView.state.selection.main.head

        // need to handle this better
        const currentNode = sourceView
          .getNodesAt(cursor)
          .filter((node) => !['Text', 'Article'].includes(node.nodeType))[0]

        if (!currentNode) {
          return
        }

        const { nodeId } = currentNode
        // if cursor is in new node, create the new el
        if (nodeId !== this.currentNodeId) {
          this.currentNodeId = nodeId
          if (this.currentInfoBox) {
            this.currentInfoBox.remove()
          }
          const domNode = sourceView.domElement.value.querySelector(
            `#${nodeId}`
          )
          if (!domNode) {
            return
          }
          this.currentInfoBox = domNode.cloneNode(true) as HTMLElement

          this.currentInfoBox.setAttribute('id', `info-box-${nodeId}`)
          this.currentInfoBox.style.position = 'block'
          this.currentInfoBox.style.width = '100%'
          this.dom.appendChild(this.currentInfoBox)
        }

        // reposition the infobox on the y-axis
        currentView.requestMeasure({
          read: (view) => {
            if (this.currentInfoBox) {
              const { top } = view.coordsAtPos(cursor)

              // skip if cursor y position hasn't changed
              if (this.cursorY === top) {
                return
              }
              this.cursorY = top
              const editorTop = view.scrollDOM.getBoundingClientRect().top
              const editorBottom = view.dom.getBoundingClientRect().bottom
              const yOffset =
                this.currentInfoBox.getBoundingClientRect().height / 2

              let yPos = top - editorTop - yOffset + view.defaultLineHeight / 2

              if (yPos < 0) {
                yPos = 2
              }
              if (
                yPos + this.currentInfoBox.getBoundingClientRect().height >
                editorBottom
              ) {
                yPos =
                  editorBottom -
                  this.currentInfoBox.getBoundingClientRect().height
              }
              this.currentInfoBox.style.top = `${yPos}px`
            }
          },
        })
      }

      destroy() {
        if (this.currentInfoBox) {
          this.currentInfoBox.remove()
        }
        this.dom.remove()
      }
    }
  )

  return infoSideBarPlugin
}

/**
 * Override the scoller elements width to allow space for the sidebar
 */
const editorStyles = EditorView.theme({
  '.cm-scroller': {
    maxWidth: '70%',
    height: 'content',
  },
})

const infoSideBar = (sourceView: SourceView): Extension => [
  infoSideBarExtension(sourceView),
  editorStyles,
]

export { infoSideBar }
