$Version = "0.2.0"
$Arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString().ToLower()
if ($env:TARGET_ARCH) { $Arch = $env:TARGET_ARCH }
$DistName = "zapnano-$Version-windows-$Arch"
$DistDir = "distribution\windows\dist\$DistName"

Write-Host "Packaging ZapNano $Version for Windows ($Arch)..."

# Ensure we are in root directory
Push-Location (Split-Path -Parent $MyInvocation.MyCommand.Path)
cd ..\..

# Build release if not built
if (!(Test-Path "target\release\zapnano.exe") -and !(Test-Path "target\$env:CARGO_BUILD_TARGET\release\zapnano.exe")) {
    Write-Host "Building release binary..."
    if ($env:CARGO_BUILD_TARGET) {
        cargo build --release --target $env:CARGO_BUILD_TARGET
    } else {
        cargo build --release
    }
}

$Binary = "target\release\zapnano.exe"
if ($env:CARGO_BUILD_TARGET -and (Test-Path "target\$env:CARGO_BUILD_TARGET\release\zapnano.exe")) {
    $Binary = "target\$env:CARGO_BUILD_TARGET\release\zapnano.exe"
}

Write-Host "Cleaning and preparing dist directory..."
Remove-Item -Path "distribution\windows\dist" -Recurse -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path "$DistDir\resources" | Out-Null

Write-Host "Copying binary..."
Copy-Item -Path $Binary -Destination "$DistDir\zapnano.exe" -Force

if (Test-Path "extensions") {
    Write-Host "Copying extensions..."
    Copy-Item -Path "extensions" -Destination "$DistDir\resources" -Recurse -Force
}

Write-Host "Copying installer script..."
Copy-Item -Path "distribution\windows\install.ps1" -Destination "$DistDir\" -Force

Write-Host "Archiving into .zip..."
Compress-Archive -Path "$DistDir" -DestinationPath "distribution\windows\dist\$DistName.zip" -Force

Pop-Location
Write-Host "Distribution package ready at: distribution\windows\dist\$DistName.zip"
