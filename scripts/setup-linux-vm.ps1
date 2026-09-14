# Create an Ubuntu 24.04 VirtualBox VM for Linux desktop installer testing.
# Requires: Administrator approval for VirtualBox install; manual Ubuntu install on first boot.
# Usage (PowerShell, repo root): .\scripts\setup-linux-vm.ps1

$ErrorActionPreference = "Stop"

$VmName = "azure-portfolio-linux"
$IsoUrl = "https://releases.ubuntu.com/24.04/ubuntu-24.04.3-desktop-amd64.iso"
$IsoDir = Join-Path $env:USERPROFILE "Downloads"
$IsoPath = Join-Path $IsoDir "ubuntu-24.04.3-desktop-amd64.iso"
$RepoRoot = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
$ShareDir = Join-Path $RepoRoot "linux-test-artifacts"

function Ensure-VirtualBox {
  $vbox = Get-Command VBoxManage -ErrorAction SilentlyContinue
  if ($vbox) {
    return $vbox.Source
  }
  Write-Host "Installing VirtualBox via winget ..." -ForegroundColor Cyan
  winget install --id Oracle.VirtualBox -e --accept-package-agreements --accept-source-agreements
  $vbox = Get-Command VBoxManage -ErrorAction SilentlyContinue
  if (-not $vbox) {
    Write-Error "VBoxManage not on PATH. Reopen PowerShell after VirtualBox install."
  }
  return $vbox.Source
}

function Ensure-UbuntuIso {
  if (Test-Path $IsoPath) {
    Write-Host "ISO already present: $IsoPath" -ForegroundColor Green
    return
  }
  New-Item -ItemType Directory -Force -Path $IsoDir | Out-Null
  Write-Host "Downloading Ubuntu 24.04 ISO (~6 GB) to $IsoPath ..." -ForegroundColor Cyan
  Invoke-WebRequest -Uri $IsoUrl -OutFile $IsoPath -UseBasicParsing
}

function Ensure-SharedFolder {
  New-Item -ItemType Directory -Force -Path $ShareDir | Out-Null
  Write-Host "Shared folder (copy installers here from Windows): $ShareDir" -ForegroundColor Green
}

$VBoxManage = Ensure-VirtualBox
Ensure-UbuntuIso
Ensure-SharedFolder

$existing = & $VBoxManage list vms 2>$null | Select-String "`"$VmName`""
if ($existing) {
  Write-Host "VM '$VmName' already exists. Start it from VirtualBox." -ForegroundColor Yellow
  exit 0
}

Write-Host "Creating VM '$VmName' ..." -ForegroundColor Cyan
& $VBoxManage createvm --name $VmName --ostype "Ubuntu_64" --register
& $VBoxManage modifyvm $VmName --memory 4096 --cpus 2 --vram 128 --graphicscontroller vmsvga
& $VBoxManage modifyvm $VmName --nic1 nat
& $VBoxManage createhd --filename (Join-Path $env:USERPROFILE "VirtualBox VMs\$VmName\$VmName.vdi") --size 40960
& $VBoxManage storagectl $VmName --name "SATA" --add sata --controller IntelAhci
& $VBoxManage storageattach $VmName --storagectl "SATA" --port 0 --device 0 --type hdd --medium (Join-Path $env:USERPROFILE "VirtualBox VMs\$VmName\$VmName.vdi")
& $VBoxManage storageattach $VmName --storagectl "SATA" --port 1 --device 0 --type dvddrive --medium $IsoPath
& $VBoxManage sharedfolder add $VmName --name "linux-test-artifacts" --hostpath $ShareDir --automount

Write-Host ""
Write-Host "VM created. Next steps:" -ForegroundColor Green
Write-Host "  1. Open VirtualBox and start '$VmName'"
Write-Host "  2. Install Ubuntu 24.04 from the attached ISO"
Write-Host "  3. In the guest: Devices -> Shared Folders -> mount linux-test-artifacts"
Write-Host "  4. On Windows: .\scripts\download-linux-desktop.ps1"
Write-Host "  5. In Ubuntu guest: bash /path/to/shared/test-linux-desktop.sh"
