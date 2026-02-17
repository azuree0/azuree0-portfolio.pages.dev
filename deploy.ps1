# Build and deploy portfolio to Cloudflare Pages.
# Requires: CLOUDFLARE_API_TOKEN, CLOUDFLARE_ACCOUNT_ID (env vars or .env)
# Or: push to main and let GitHub Actions deploy (needs repo secrets).

$ErrorActionPreference = "Stop"

# Build
Write-Host "Building with Trunk..." -ForegroundColor Cyan
$env:NO_COLOR = $null
trunk build --release --public-url /
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

# Deploy (local) - needs Node + wrangler + Cloudflare credentials
if ($env:CLOUDFLARE_API_TOKEN -and $env:CLOUDFLARE_ACCOUNT_ID) {
    Write-Host "Deploying to Cloudflare Pages..." -ForegroundColor Cyan
    npx wrangler pages deploy dist --project-name=azuree0-portfolio
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Deployed. Site: https://azuree0-portfolio.pages.dev" -ForegroundColor Green
    }
} else {
    Write-Host "Skipping local deploy (no CLOUDFLARE_API_TOKEN / CLOUDFLARE_ACCOUNT_ID)." -ForegroundColor Yellow
    Write-Host "Either set env vars and re-run, or push to main for GitHub Actions deploy." -ForegroundColor Yellow
}
