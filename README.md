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

**Git**

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

**Deploy to Cloudflare Pages**

- **Live** — https://azuree0-portfolio.pages.dev/
- **Domains** — https://dash.cloudflare.com/f1eeae10e7537ebbaef3bc34f93ab59d/home/domains

1. In Cloudflare: **Workers & Pages** → **Create** → **Pages** → **Direct Upload**:
   - **Create project** → name: (must match workflow). Drag-and-drop any small file (e.g. `index.html`) to create the project; GitHub Actions will overwrite on first deploy.

2. Get credentials: Dashboard → **Account ID** (right sidebar). **My Profile** → **API Tokens** → **Create Custom Token** → restrict to this repo only:
   - **Permissions:** Account → Cloudflare Pages → Edit.
   - **Account resources:** Include → **only your account** (not "All accounts").
   - Use this token

3. Add secrets: `CLOUDFLARE_ACCOUNT_ID`, `CLOUDFLARE_API_TOKEN` (pick one):
   - **Dashboard:** → New repository secret.
   - **CLI:** `gh auth login` then `.\setup-github-secrets.ps1 -AccountId "YOUR_ID" -ApiToken "YOUR_TOKEN"`

4. Push to `main` (or `master`). The workflow builds with Trunk and deploys `dist/` to Cloudflare Pages.

# Function

```text
┌─────────────────────────────────────────────────────────────────┐
│ BROWSER                                                         │
│ • Full-page WebGL2 canvas (underwater particles)                │
│ • Yew UI overlay (hero, repo grid, footer)                      │
│ • Perf: critical inline CSS, dns-prefetch, preload Three.js      │
│ • Repo hover → link prefetch (GitHub); lazy images; CF cache     │
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
│       ├── deploy.yml # GitHub Actions: build + deploy to GitHub Pages
│       └── deploy-cloudflare-pages.yml # GitHub Actions: build + deploy to Cloudflare Pages 
├── Cargo.toml # Rust project config                                (Config)
├── Dockerfile # Docker build for Render                            (Config)
├── nginx.conf # Nginx config for static serve                      (Config)
├── render.yaml # Render service config                             (Config)
├── deploy.ps1 # Build + deploy to Cloudflare (local)               (Config)
├── setup-github-secrets.ps1 # Add CLOUDFLARE_* secrets via gh CLI  (Config)
├── Trunk.toml # WASM build config                                  (Config)
├── index.html # Entry HTML + critical CSS + resource hints         (Config)
├── README.md # This file
├── static/
│   └── _headers # Cloudflare Pages cache + security headers        (Config)
├── styles/
│   └── main.css # Underwater theme                                 (Frontend)
└── src/
    ├── main.rs # Yew mount                                         (Backend)
    ├── lib.rs # Crate root                                         (Backend)
    ├── prefetch.rs # link rel=prefetch on repo hover               (Frontend)
    ├── app.rs # Root App component                                 (Frontend)
    ├── scene.rs # WebGL2 underwater particle scene                 (Frontend)
    ├── components/
    │   ├── mod.rs
    │   ├── hero.rs # Hero section                                  (Frontend)
    │   ├── repo_grid.rs # Repo grid                                (Frontend)
    │   └── repo_card.rs # Repo card                                (Frontend)
    ├── models/
    │   └── repo.rs # Repo struct                                   (Backend)
    └── api/
        └── github.rs # GitHub API + cache                          (Backend)
```
