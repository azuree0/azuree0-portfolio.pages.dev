# Prior

## Rust

### Install

- Rust — [https://rustup.rs/](https://rustup.rs/)

```
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli
```

### Build

```
trunk build --release
```

### Run

```
trunk serve
```

## Electron

### Install

- Node.js LTS — [https://nodejs.org/en/download](https://nodejs.org/en/download)

```
npm install
```

### Build

`npm run make` runs `build:electron` (TypeScript main/preload), Trunk WASM, then Electron Forge.

```
npm run make
```

### Run

```
npm start
```

### Test Linux (.AppImage / .deb)

From Windows, use **WSL Ubuntu**, a **VirtualBox / VMware Ubuntu VM**, or a **Cursor Cloud Agent** (Linux). On the Linux side:

**Install (Linux host only)**

```
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli
npm install
sudo apt-get update && sudo apt-get install -y squashfs-tools
```

`npm run run:linux-appimage` installs Electron runtime libraries on Debian/Ubuntu if needed (`libnss3`, `libgtk-3-0`, etc.).

**Build test artifacts (.deb + .AppImage)**

```
npm run make:linux-test
```

**Run AppImage**

```
npm run run:linux-appimage
```

**WSL from Windows (PowerShell, repo root)**

```
wsl -d Ubuntu -- bash -lc "cd '/mnt/c/Users/Azure/Documents/Axiom/Rust/azuree0-portfolio.pages.dev-' && npm run make:linux-test"
```



# Function

```text
┌─────────────────────────────────────────────────────────────────┐
│ BROWSER                                                         │
│ • Full-page WebGL2 canvas (underwater particles)                │
│ • Yew UI overlay (hero, repo grid, footer)                      │
│ • Perf: critical inline CSS, dns-prefetch, preload Three.js     │
│ • Repo hover → link prefetch (GitHub); lazy images; CF cache    │
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
├── .github/
│   └── workflows/
│       ├── deploy.yml                  # GitHub Actions: deploy to GitHub Pages
│       ├── deploy-cloudflare-pages.yml # GitHub Actions: deploy to Cloudflare Pages
│       └── release-desktop.yml         # Tag v*: Electron installers to GitHub Releases
├── backend/
│   ├── Main.ts                         # Electron main process (source)                  (Backend)
│   ├── Preload.ts                      # contextBridge desktop flag (source)             (Backend)
│   ├── Main.js                         # tsc emit (gitignored)                           (Backend)
│   └── Preload.js                      # tsc emit (gitignored)                           (Backend)
├── assets/
│   └── icon.png                        # App icon for Electron packager                  (Frontend)
├── forge.config.mjs                    # Electron Forge makers + GitHub publisher          #
├── package.json                        # Electron scripts: start, make, publish          #
├── Cargo.toml                          # Rust project configuration
├── Dockerfile                          # Image for Render
├── nginx.conf                          # Static file serving
├── render.yaml                         # Render service definition
├── deploy.ps1                          # Local build + Cloudflare deploy
├── scripts/
│   ├── make-linux-test.sh              # Linux .deb + .AppImage (WSL / VM / Cloud Agent)
│   └── run-linux-appimage.sh           # Launch built AppImage
├── setup-github-secrets.ps1            # CLOUDFLARE_* secrets via gh CLI
├── Trunk.toml                          # WASM build
├── index.html                          # Entry HTML, critical CSS, hints
├── README.md
├── static/
│   ├── _headers                        # Pages cache + security headers
│   ├── og-image.png                    # Open Graph / Twitter image
│   ├── three.min.js                    # Three.js r128 (local; desktop offline)          (Frontend)
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

**First-time: Git**

```
git init
git add .
git commit -m "Add Render deployment"
git config --global user.email "your-email@example.com"
git config --global user.name "Your Name"
git remote add origin https://github.com/yourusername/portfolio.git
git branch -M main
git push -u origin main
```

**First-time: Cloudflare Pages project + Actions**

1. In Cloudflare: **Workers & Pages** → **Create** → **Pages** → **Direct Upload**:
  - **Create project** → name must match the workflow. Drag-and-drop any small file (e.g. `index.html`) so the project exists; GitHub Actions will overwrite on first deploy.
2. Note **Account ID** (Cloudflare dashboard right sidebar). **My Profile** → **API Tokens** → **Create Custom Token** → restrict to this repo only:
  - **Permissions:** Account → Cloudflare Pages → Edit.
  - **Account resources:** Include → **only your account** (not “All accounts”).
3. Add GitHub repository secrets `CLOUDFLARE_ACCOUNT_ID` and `CLOUDFLARE_API_TOKEN`:
  - **Dashboard:** GitHub repo → **Settings** → **Secrets and variables** → **Actions** → **New repository secret**.
  - **CLI:** `gh auth login` then `.\setup-github-secrets.ps1 -AccountId "YOUR_ID" -ApiToken "YOUR_TOKEN"`
4. Push to `main` (or `master`). The workflow builds with Trunk and deploys `dist/` to Cloudflare Pages.

**Routine: change the app and refresh production (Windows / PowerShell, repo root)**

1. Edit `src\`, `styles\`, `static\`, `index.html`, etc.
2. Build and optional local preview (same commands as **# Prior**):

**Build**

```
trunk build --release
```

**Run** (optional)

```
trunk serve
```

1. Push to GitHub:

```
git add .
git commit -m "Describe your change"
git push origin main
```

1. Deploy: push to `main` runs **Deploy to Cloudflare Pages** automatically. Manual: **Actions** → **Deploy to Cloudflare Pages** → **Run workflow** → branch **main**.
2. Verify production URL and Pages dashboard (links above).

**Desktop release (GitHub Releases)**

1. Bump `version` in `package.json` if needed.
2. Tag and push:

```
git tag v0.1.0
git push origin v0.1.0
```

1. **Actions** runs **Release Desktop** on `windows-latest`, `macos-latest`, and `ubuntu-latest` (parallel); uploads installers to a **draft** release on [github.com/azuree0/azuree0-portfolio.pages.dev/releases](https://github.com/azuree0/azuree0-portfolio.pages.dev/releases).
2. Review assets (Windows `.exe`, macOS `.dmg`, Linux `.AppImage` and `.deb`), then **Publish release** on GitHub.
3. Local Windows build (optional): `npm run make` outputs to `out/make/`.

**Secrets (first-time or rotate token only)**

- Custom token: **Account** → **Cloudflare Pages** → **Edit**; **Account resources** = this account only.
- **Secret names** (exact): `CLOUDFLARE_API_TOKEN` = paste token in **Secret** field; `CLOUDFLARE_ACCOUNT_ID` = paste **Account ID** (32-character hex from Cloudflare sidebar or from URL `https://dash.cloudflare.com/<Account_ID>/...`).

**Deploy failed**

- Open the failed job on the workflow URL above; read the **Deploy to Cloudflare Pages** step log.
- Revoke a leaked token at the API Tokens URL; add a new token; update only `CLOUDFLARE_API_TOKEN` in repository secrets.

