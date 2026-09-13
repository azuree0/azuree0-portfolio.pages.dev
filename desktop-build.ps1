# Build Electron desktop installer for the current OS (outputs to desktop/out/make/).
$ErrorActionPreference = "Stop"

Push-Location desktop
try {
    if (-not (Test-Path node_modules)) {
        Write-Host "Installing desktop dependencies..." -ForegroundColor Cyan
        npm install
        if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    }

    Write-Host "Building desktop installer..." -ForegroundColor Cyan
    $env:NO_COLOR = "false"
    npm run make
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

    Write-Host "Installers in desktop\out\make\" -ForegroundColor Green
} finally {
    Pop-Location
}
