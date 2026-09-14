import { app, BrowserWindow } from 'electron/main'
import path from 'node:path'
import fs from 'node:fs'
import os from 'node:os'
import squirrelStartup from 'electron-squirrel-startup'

// Windows Squirrel installer: exit early during install/update hooks.
if (squirrelStartup) {
  app.quit()
}

let mainWindow: BrowserWindow | null = null

// Dev: separate userData so stale Electron processes do not lock the disk cache (Windows).
function configureDevCachePaths(): void {
  if (app.isPackaged) {
    return
  }

  const devUserData = path.join(os.tmpdir(), 'azure-portfolio-dev')
  const diskCacheDir = path.join(devUserData, 'disk-cache')

  fs.mkdirSync(diskCacheDir, { recursive: true })
  app.setPath('userData', devUserData)
  app.commandLine.appendSwitch('disk-cache-dir', diskCacheDir)
}

configureDevCachePaths()

// Resolves Trunk dist/: packaged extraResource vs repo-root dist during dev.
function distDir(): string {
  if (app.isPackaged) {
    return path.join(process.resourcesPath, 'dist')
  }

  return path.join(__dirname, '..', 'dist')
}

// UI: create the main window and load the Trunk-built portfolio.
function createWindow(): void {
  mainWindow = new BrowserWindow({
    width: 1280,
    height: 800,
    minWidth: 800,
    minHeight: 600,
    backgroundColor: '#0a0a0f',
    show: false,
    webPreferences: {
      preload: path.join(__dirname, 'Preload.js'),
      contextIsolation: true,
      nodeIntegration: false
    }
  })

  mainWindow.once('ready-to-show', () => {
    mainWindow?.show()
  })

  mainWindow.loadFile(path.join(distDir(), 'index.html'))

  mainWindow.on('closed', () => {
    mainWindow = null
  })
}

app.whenReady().then(() => {
  createWindow()

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow()
    }
  })
})

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit()
  }
})
