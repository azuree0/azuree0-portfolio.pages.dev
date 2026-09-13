# Local dev server for the portfolio (hot reload via Trunk).
$ErrorActionPreference = "Stop"

Write-Host "Starting Trunk dev server..." -ForegroundColor Cyan
$env:NO_COLOR = $null
trunk serve
