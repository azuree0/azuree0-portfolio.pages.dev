# Prior

**Install**

- Rust — https://rustup.rs/

```
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli
```

**Build**

```
trunk build --release
```

**Run**

```
trunk serve
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
│       └── deploy-cloudflare-pages.yml # GitHub Actions: deploy to Cloudflare Pages
├── Cargo.toml                          # Rust project configuration
├── Dockerfile                          # Image for Render
├── nginx.conf                          # Static file serving
├── render.yaml                         # Render service definition
├── deploy.ps1                          # Local build + Cloudflare deploy
├── setup-github-secrets.ps1            # CLOUDFLARE_* secrets via gh CLI
├── Trunk.toml                          # WASM build
├── index.html                          # Entry HTML, critical CSS, hints
├── README.md
├── static/
│   ├── _headers                        # Pages cache + security headers
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

**Repository**

- **GitHub** —                    https://github.com/azuree0/azuree0-portfolio.pages.dev

**Live and dashboards**

- **Production** —                https://azuree0-portfolio.pages.dev/
- **Cloudflare domains** —        https://dash.cloudflare.com/f1eeae10e7537ebbaef3bc34f93ab59d/home/domains
- **Cloudflare Pages project** —  https://dash.cloudflare.com/f1eeae10e7537ebbaef3bc34f93ab59d/pages/view/azuree0-portfolio
- **GitHub Actions workflow** —   https://github.com/azuree0/azuree0-portfolio.pages.dev/actions/workflows/deploy-cloudflare-pages.yml
- **Repository secrets** —        https://github.com/azuree0/azuree0-portfolio.pages.dev/settings/secrets/actions
- **Create API token** —          https://dash.cloudflare.com/profile/api-tokens


**First-time: Git (optional, new clone)**

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

3. Push to GitHub:

```
git add .
git commit -m "Describe your change"
git push origin main
```


4. Deploy: push to `main` runs **Deploy to Cloudflare Pages** automatically. Manual: **Actions** → **Deploy to Cloudflare Pages** → **Run workflow** → branch **main**.

5. Verify production URL and Pages dashboard (links above).

**Secrets (first-time or rotate token only)**

- Custom token: **Account** → **Cloudflare Pages** → **Edit**; **Account resources** = this account only.
- **Secret names** (exact): `CLOUDFLARE_API_TOKEN` = paste token in **Secret** field; `CLOUDFLARE_ACCOUNT_ID` = paste **Account ID** (32-character hex from Cloudflare sidebar or from URL `https://dash.cloudflare.com/<Account_ID>/...`).

**Deploy failed**

- Open the failed job on the workflow URL above; read the **Deploy to Cloudflare Pages** step log.
- Revoke a leaked token at the API Tokens URL; add a new token; update only `CLOUDFLARE_API_TOKEN` in repository secrets.
