import { MakerAppImage } from '@reforged/maker-appimage'
import { FusesPlugin } from '@electron-forge/plugin-fuses'
import { FuseV1Options, FuseVersion } from '@electron/fuses'

// Paths excluded from app.asar (Trunk dist ships via extraResource instead).
const ASAR_IGNORE_PREFIXES = [
  '/src',
  '/target',
  '/dist',
  '/out',
  '/.git',
  '/.github',
  '/debug',
  '/node_modules',
  '/static',
  '/styles',
  '/emsdk',
  '/.wrangler',
  '/.cursor'
]

function shouldIgnoreAsarPath(filePath) {
  if (!filePath) {
    return false
  }

  if (/^\/backend\/.*\.ts$/.test(filePath)) {
    return true
  }

  return ASAR_IGNORE_PREFIXES.some(
    (prefix) => filePath === prefix || filePath.startsWith(`${prefix}/`)
  )
}

export default {
  packagerConfig: {
    name: 'Azure Portfolio',
    executableName: 'azure-portfolio',
    asar: true,
    icon: './assets/icon',
    extraResource: ['./dist'],
    appBundleId: 'com.azuree0.portfolio',
    ignore: shouldIgnoreAsarPath
  },
  rebuildConfig: {},
  makers: [
    {
      name: '@electron-forge/maker-squirrel',
      platforms: ['win32'],
      config: {
        name: 'azure-portfolio',
        authors: 'Azure'
      }
    },
    {
      name: '@electron-forge/maker-dmg',
      platforms: ['darwin']
    },
    {
      name: '@electron-forge/maker-zip',
      platforms: ['darwin']
    },
    {
      name: '@electron-forge/maker-deb',
      platforms: ['linux'],
      config: {
        options: {
          maintainer: 'Azure',
          homepage: 'https://azuree0-portfolio.pages.dev/'
        }
      }
    },
    {
      name: '@electron-forge/maker-snap',
      platforms: ['linux'],
      config: {
        summary: 'Azure portfolio desktop app',
        features: {
          audio: false,
          webgl: true
        }
      }
    },
    new MakerAppImage({
      options: {
        categories: ['Network']
      }
    })
  ],
  plugins: [
    {
      name: '@electron-forge/plugin-auto-unpack-natives',
      config: {}
    },
    new FusesPlugin({
      version: FuseVersion.V1,
      [FuseV1Options.RunAsNode]: false,
      [FuseV1Options.EnableCookieEncryption]: true,
      [FuseV1Options.EnableNodeOptionsEnvironmentVariable]: false,
      [FuseV1Options.EnableNodeCliInspectArguments]: false,
      [FuseV1Options.EnableEmbeddedAsarIntegrityValidation]: true,
      [FuseV1Options.OnlyLoadAppFromAsar]: true
    })
  ],
  publishers: [
    {
      name: '@electron-forge/publisher-github',
      config: {
        repository: {
          owner: 'azuree0',
          name: 'azuree0-portfolio.pages.dev'
        },
        prerelease: false,
        draft: true
      }
    }
  ]
}
