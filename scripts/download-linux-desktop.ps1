# Download Linux desktop installers from a GitHub Release into linux-test-artifacts/.
# Usage: .\scripts\download-linux-desktop.ps1
#        .\scripts\download-linux-desktop.ps1 -Tag v0.1.0

param(
    [string]$Tag = "",
    [string]$Repo = "azuree0/azuree0-portfolio.pages.dev"
)

$ErrorActionPreference = "Stop"
$root = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
$outDir = Join-Path $root "linux-test-artifacts"

if (-not (Get-Command gh -ErrorAction SilentlyContinue)) {
    Write-Error "GitHub CLI (gh) is required. Install from https://cli.github.com/ then run: gh auth login"
}

New-Item -ItemType Directory -Force -Path $outDir | Out-Null

$releaseArgs = @("release", "download", "--repo", $Repo, "--dir", $outDir, "--pattern", "*")
if ($Tag) {
    $releaseArgs += $Tag
} else {
    $releaseArgs += "--latest"
}

Write-Host "Downloading Linux desktop assets to $outDir ..." -ForegroundColor Cyan
& gh @releaseArgs

$assets = Get-ChildItem -Path $outDir -File -ErrorAction SilentlyContinue
if (-not $assets) {
    Write-Warning "No files downloaded. Push a tag (git tag v0.1.0; git push origin v0.1.0) and wait for Release Desktop to finish."
    exit 1
}

Write-Host "Downloaded:" -ForegroundColor Green
$assets | ForEach-Object { Write-Host "  $($_.Name) ($([math]::Round($_.Length / 1MB, 2)) MB)" }

Write-Host ""
Write-Host "Next (WSL Ubuntu):" -ForegroundColor Cyan
Write-Host "  wsl bash scripts/test-linux-desktop.sh"
