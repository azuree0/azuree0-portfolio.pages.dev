# Run portfolio in Electron for local desktop dev (rebuilds WASM first).
$ErrorActionPreference = "Stop"

Push-Location desktop
try {
    if (-not (Test-Path node_modules)) {
        Write-Host "Installing desktop dependencies..." -ForegroundColor Cyan
        npm install
        if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    }

    Write-Host "Starting Electron..." -ForegroundColor Cyan
    $env:NO_COLOR = "false"
    npm run start
} finally {
    Pop-Location
}
