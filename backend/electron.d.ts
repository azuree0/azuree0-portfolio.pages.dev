declare module 'electron/main' {
  export * from 'electron'
}

declare module 'electron/renderer' {
  export * from 'electron'
}

declare module 'electron-squirrel-startup' {
  const started: boolean
  export = started
}

declare namespace NodeJS {
  interface Process {
    resourcesPath: string
  }
}
