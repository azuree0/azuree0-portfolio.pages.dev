# Add custom domain azuree0.dev to Cloudflare Pages project azuree0-portfolio via API.
# Requires: CLOUDFLARE_API_TOKEN, CLOUDFLARE_ACCOUNT_ID (env or .env). Same as deploy.
# Run: .\add-pages-custom-domain.ps1

$ErrorActionPreference = "Stop"
$accountId = $env:CLOUDFLARE_ACCOUNT_ID
$token = $env:CLOUDFLARE_API_TOKEN
$projectName = "azuree0-portfolio"
$domain = "azuree0.dev"

if (-not $accountId -or -not $token) {
    Write-Host "Set CLOUDFLARE_ACCOUNT_ID and CLOUDFLARE_API_TOKEN (e.g. from GitHub secrets or .env)." -ForegroundColor Red
    exit 1
}

$url = "https://api.cloudflare.com/client/v4/accounts/$accountId/pages/projects/$projectName/domains"
$body = @{ name = $domain } | ConvertTo-Json

try {
    $resp = Invoke-RestMethod -Uri $url -Method Post -Headers @{
        "Authorization" = "Bearer $token"
        "Content-Type"  = "application/json"
    } -Body $body
    if ($resp.success) {
        Write-Host "Custom domain $domain added. Site: https://$domain" -ForegroundColor Green
    } else {
        Write-Host "Response: $($resp | ConvertTo-Json -Depth 5)" -ForegroundColor Yellow
    }
} catch {
    $statusCode = $_.Exception.Response.StatusCode.value__
    $reader = [System.IO.StreamReader]::new($_.Exception.Response.GetResponseStream())
    $errBody = $reader.ReadToEnd()
    Write-Host "API error ($statusCode): $errBody" -ForegroundColor Red
    exit 1
}
