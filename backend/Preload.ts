import { contextBridge } from 'electron/renderer'

// Expose a minimal desktop flag for optional renderer branching.
contextBridge.exposeInMainWorld('desktop', {
  isDesktop: true
})
