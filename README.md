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

**Deploy to Cloudflare Pages (free)**

- **Workers & Pages**: https://dash.cloudflare.com/f1eeae10e7537ebbaef3bc34f93ab59d/workers-and-pages
- **Builds (MCP)**: https://builds.mcp.cloudflare.com/mcp

1. In Cloudflare: **Workers & Pages** → **Create** → **Pages** → **Connect to Git** (optional). For this repo, use **Direct Upload** so GitHub Actions does the build:
   - **Create project** → **Direct Upload** → name: `portfolio` (must match workflow `--project-name=portfolio`). Skip upload; the first push will deploy.
2. Get credentials: Dashboard → **Account ID** (right sidebar); **My Profile** → **API Tokens** → Create Token → use "Edit Cloudflare Workers" + "Cloudflare Pages — Edit" (or Custom: Account + Pages Edit).
3. In the repo: **Settings** → **Secrets and variables** → **Actions** → New repository secret:
   - `CLOUDFLARE_ACCOUNT_ID` = your Account ID
   - `CLOUDFLARE_API_TOKEN` = your API token
4. Push to `main` (or `master`). The workflow builds with Trunk and deploys `dist/` to Cloudflare Pages.
5. Site at `https://portfolio.pages.dev` (or the URL in **Workers & Pages** → **portfolio**). Optionally add a custom domain there.

# Function

```text
┌─────────────────────────────────────────────────────────────────┐
│ BROWSER                                                         │
│ • Full-page WebGL2 canvas (underwater particles)                │
│ • Yew UI overlay (hero, repo grid, footer)                      │
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
│       └── deploy-cloudflare-pages.yml # GitHub Actions: build + deploy to Cloudflare Pages (free)
├── Cargo.toml # Rust project config                                (Config)
├── Dockerfile # Docker build for Render                            (Config)
├── nginx.conf # Nginx config for static serve                      (Config)
├── render.yaml # Render service config                             (Config)
├── Trunk.toml # WASM build config                                  (Config)
├── index.html # Entry HTML                                         (Config)
├── README.md # This file
├── styles/
│   └── main.css # Underwater theme                                 (Frontend)
└── src/
    ├── main.rs # Yew mount                                         (Backend)
    ├── lib.rs # Crate root                                         (Backend)
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
