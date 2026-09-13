# Build portfolio WASM for web preview (outputs to dist/).
$ErrorActionPreference = "Stop"

Write-Host "Building with Trunk..." -ForegroundColor Cyan
$env:NO_COLOR = $null
trunk build --release --public-url /
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "Built dist/. Preview with: .\serve.ps1" -ForegroundColor Green
