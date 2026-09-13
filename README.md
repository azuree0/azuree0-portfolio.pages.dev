# Prior

**Install** (Windows / PowerShell, repo root)

- Rust — https://rustup.rs/
- Node.js — https://nodejs.org/ (desktop only)

```
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli
```

**Build** (web)

```
.\build.ps1
```

Or manually:

```
trunk build --release
```

**Run** (web — local dev server)

```
.\serve.ps1
```

Or manually:

```
trunk serve
```

Open http://127.0.0.1:8080/ in your browser.

**Desktop** (Electron — online-only; CDN Three.js/fonts + GitHub API need network)

First time:

```
cd desktop
npm install
cd ..
```

Dev:

```
.\desktop-dev.ps1
```

Installer for the current OS:

```
.\desktop-build.ps1
```

Installers are written to `desktop\out\make\`. Cached repos may still display from `localStorage` when offline.

**Clone on a new machine**

```
git clone https://github.com/azuree0/azuree0-portfolio.pages.dev.git
cd azuree0-portfolio.pages.dev
```

Then run the install steps above.

# Function

```text
┌─────────────────────────────────────────────────────────────────┐
│ BROWSER / ELECTRON (local)                                      │
│ • Full-page WebGL2 canvas (underwater particles)                │
│ • Yew UI overlay (hero, repo grid, footer)                      │
│ • Perf: critical inline CSS, dns-prefetch, preload Three.js     │
│ • Repo hover → link prefetch (GitHub); lazy images              │
│ • Desktop: Electron shell serves Trunk dist over localhost      │
└─────────────────────────────────────────────────────────────────┘
                                    ▼
┌─────────────────────────────────────────────────────────────────┐
│ RUST WASM                                                       │
│ • Yew: App, Hero, RepoGrid, RepoCard                            │
│ • Scene: WebGL2 particle renderer                               │
│ • API: GitHub fetch, localStorage cache, periodic refresh       │
└─────────────────────────────────────────────────────────────────┘
                                    ▼
┌─────────────────────────────────────────────────────────────────┐
│ GITHUB API                                                      │
│ • GET /users/repos                                              │
└─────────────────────────────────────────────────────────────────┘
```

# Structure

```text
portfolio/
├── build.ps1                           # Local web build → dist/
├── serve.ps1                           # Local Trunk dev server
├── desktop-build.ps1                   # Electron installer (current OS)
├── desktop-dev.ps1                     # Electron dev window
├── Cargo.toml                          # Rust project configuration
├── Trunk.toml                          # WASM build (web, public_url /)
├── Trunk.desktop.toml                  # WASM build for Electron (public_url ./)
├── desktop/
│   ├── package.json                    # Electron Forge scripts (start, make)
│   ├── main.js                         # Main process + localhost static server
│   ├── preload.js                      # Renderer preload (context isolation)
│   └── forge.config.js                 # Packager + OS makers (.exe, .dmg, etc.)
├── index.html                          # Entry HTML, critical CSS, hints
├── README.md
├── static/
│   ├── og-image.png                    # Open Graph / Twitter image
│   └── icosahedron-overlay.js          # Tagline-hover icosahedron                   (Frontend)
├── styles/
│   └── main.css                        # Underwater theme                            (Frontend)
└── src/
    ├── main.rs                         # Yew mount                                   (Backend)
    ├── lib.rs                          # Crate root                                  (Backend)
    ├── prefetch.rs                     # link prefetch on repo hover                 (Frontend)
    ├── app.rs                          # Root App                                    (Frontend)
    ├── scene.rs                        # WebGL2 particle scene                       (Frontend)
    ├── components/
    │   ├── mod.rs
    │   ├── hero.rs                     # Hero section                                (Frontend)
    │   ├── repo_grid.rs                # Repo grid                                   (Frontend)
    │   └── repo_card.rs                # Repo card                                   (Frontend)
    ├── models/
    │   └── repo.rs                     # Repo struct                                 (Backend)
    └── api/
        └── github.rs                   # GitHub API + cache                          (Backend)
```

# SOP

**Routine: change the app locally (Windows / PowerShell, repo root)**

1. Edit `src\`, `styles\`, `static\`, `index.html`, etc.

2. Preview in browser:

```
.\serve.ps1
```

3. Or preview as desktop app:

```
.\desktop-dev.ps1
```

4. Production web build (static files in `dist\`):

```
.\build.ps1
```

5. Desktop installer (`.exe` on Windows):

```
.\desktop-build.ps1
```

6. Commit when ready:

```
git add .
git commit -m "Describe your change"
git push origin main
```

**Troubleshooting**

- Trunk `NO_COLOR` error on Windows: run `$env:NO_COLOR = $null` before `trunk` commands, or use the `.ps1` scripts above.
- Desktop needs network for CDN assets and GitHub API; offline shows cached repos only.
- First Electron run: `cd desktop && npm install`.
