# Add CLOUDFLARE_ACCOUNT_ID and CLOUDFLARE_API_TOKEN to GitHub repo secrets.
# Requires: gh CLI (winget install GitHub.cli) and: gh auth login
# Usage: .\setup-github-secrets.ps1 -AccountId "YOUR_ACCOUNT_ID" -ApiToken "YOUR_API_TOKEN"

param(
    [Parameter(Mandatory=$true)][string]$AccountId,
    [Parameter(Mandatory=$true)][string]$ApiToken
)

$repo = "azuree0/Portfolio"

Write-Host "Setting GitHub secrets for $repo..." -ForegroundColor Cyan
gh secret set CLOUDFLARE_ACCOUNT_ID --body $AccountId --repo $repo
gh secret set CLOUDFLARE_API_TOKEN --body $ApiToken --repo $repo
Write-Host "Done. Push to main to trigger deploy." -ForegroundColor Green
