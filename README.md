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

**Deploy to Render**

1. Push code to GitHub
2. Sign up at https://render.com
3. New → Web Service → Connect repo
4. Render auto-detects `render.yaml` (Docker)
5. Deploy; site at `https://portfolio.onrender.com`
6. CMD ["nginx", "-g", "daemon off;"]

**Deploy to Cloudflare Pages (free)**

1. Push code to GitHub.
2. Sign up at https://dash.cloudflare.com and get:
   - **Account ID**: Dashboard → right sidebar (under "API").
   - **API token**: My Profile → API Tokens → Create Token → "Edit Cloudflare Workers" template, then add "Cloudflare Pages — Edit" (or use "Custom token" with Account + Cloudflare Pages Edit).
3. In the repo: **Settings → Secrets and variables → Actions** → New repository secret:
   - `CLOUDFLARE_ACCOUNT_ID` = your Account ID
   - `CLOUDFLARE_API_TOKEN` = your API token
4. Push to `main` (or `master`). The workflow builds with Trunk and deploys to Cloudflare Pages.
5. Site will be at `https://portfolio.pages.dev` (or the URL shown in Cloudflare Dashboard → Pages → portfolio).

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
