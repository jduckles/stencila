/**
 * Configuration of Shoelace
 */

// TODO: These imports would be better to be done where needed
import '@shoelace-style/shoelace/dist/components/button/button.js'
import '@shoelace-style/shoelace/dist/components/divider/divider.js'
import '@shoelace-style/shoelace/dist/components/dropdown/dropdown.js'
import '@shoelace-style/shoelace/dist/components/icon-button/icon-button.js'
import '@shoelace-style/shoelace/dist/components/icon/icon.js'
import '@shoelace-style/shoelace/dist/components/input/input.js'
import '@shoelace-style/shoelace/dist/components/menu-item/menu-item.js'
import '@shoelace-style/shoelace/dist/components/menu/menu.js'
import '@shoelace-style/shoelace/dist/components/tree-item/tree-item.js'
import '@shoelace-style/shoelace/dist/components/tree/tree.js'

import { setBasePath } from '@shoelace-style/shoelace/dist/utilities/base-path.js'
import { registerIconLibrary } from '@shoelace-style/shoelace/dist/utilities/icon-library.js'

import { version } from '../package.json'

const { NODE_ENV } = process.env
const base = NODE_ENV === 'development' ? 'dev' : version

setBasePath(`/~static/${base}/shoelace-style/`)

registerIconLibrary('stencila', {
  resolver: (name) => `/~static/${base}/app-assets/icons/${name}.svg`,
})

registerIconLibrary('boxicons', {
  resolver: (name) => {
    let folder = 'regular'
    if (name.substring(0, 4) === 'bxs-') folder = 'solid'
    if (name.substring(0, 4) === 'bxl-') folder = 'logos'
    return `https://cdn.jsdelivr.net/npm/boxicons@2.1.4/svg/${folder}/${name}.svg`
  },
  mutator: (svg) => svg.setAttribute('fill', 'currentColor'),
})

registerIconLibrary('lucide', {
  resolver: (name) =>
    `https://cdn.jsdelivr.net/npm/lucide-static@0.365.0/icons/${name}.svg`,
})

export type ShoelaceIconLibraries =
  | 'default'
  | 'stencila'
  | 'boxicons'
  | 'lucide'
