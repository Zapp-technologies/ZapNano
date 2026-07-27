Write-Host "Starting Zap Nano installation..."

$ZAPNANO_HOME = "$Home\.local\share\ZapNano"
$BIN_DIR = "$ZAPNANO_HOME\bin"
$RES_DIR = "$ZAPNANO_HOME\resources"
$USER_BIN = "$Home\.local\bin"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

if (!(Test-Path "$ScriptDir\zapnano.exe")) {
    Write-Error "Error: 'zapnano.exe' binary not found."
    Write-Error "Please ensure you run this script from the extracted archive directory."
    Exit 1
}

Write-Host "[1/4] Preparing directories..."
New-Item -ItemType Directory -Force -Path $BIN_DIR | Out-Null
New-Item -ItemType Directory -Force -Path $RES_DIR | Out-Null
New-Item -ItemType Directory -Force -Path $USER_BIN | Out-Null

Write-Host "[2/4] Copying files to $ZAPNANO_HOME..."
Copy-Item -Path "$ScriptDir\zapnano.exe" -Destination "$BIN_DIR\zapnano.exe" -Force

# Create znano wrapper command
$WrapperPath = "$BIN_DIR\znano.cmd"
@"
@echo off
"%~dp0zapnano.exe" %*
"@ | Out-File -FilePath $WrapperPath -Encoding ascii -Force

if (Test-Path "$ScriptDir\resources") {
    Copy-Item -Path "$ScriptDir\resources\*" -Destination $RES_DIR -Recurse -Force | Out-Null
}

Write-Host "[3/4] Creating symlinks in $USER_BIN..."
try {
    New-Item -ItemType SymbolicLink -Path "$USER_BIN\zapnano.exe" -Target "$BIN_DIR\zapnano.exe" -Force -ErrorAction Stop | Out-Null
    New-Item -ItemType SymbolicLink -Path "$USER_BIN\znano.cmd" -Target "$BIN_DIR\znano.cmd" -Force -ErrorAction Stop | Out-Null
} catch {
    # Fallback to lightweight batch wrappers if Developer Mode or admin privileges are missing
    "@`"$BIN_DIR\zapnano.exe`" %*" | Out-File -FilePath "$USER_BIN\zapnano.cmd" -Encoding ascii -Force
    "@`"$BIN_DIR\znano.cmd`" %*" | Out-File -FilePath "$USER_BIN\znano.cmd" -Encoding ascii -Force
}

$UserPath = [Environment]::GetEnvironmentVariable("Path", [EnvironmentVariableTarget]::User)
$ParsedUserPath = $UserPath -split ';' | ForEach-Object { $_.TrimEnd('\') }

if ($ParsedUserPath -notcontains $CleanUserBin) {
    Write-Host "Adding $USER_BIN to User PATH..."
    try {
        $NewUserPath = if ($UserPath -and $UserPath.Trim()) { "$UserPath;$USER_BIN" } else { $USER_BIN }
        [Environment]::SetEnvironmentVariable("Path", $NewUserPath, [EnvironmentVariableTarget]::User)
        $env:Path = "$USER_BIN;$env:Path"
        Write-Host "User PATH updated successfully. Restart your terminal session for changes to propagate fully."
    } catch {
        Write-Host "Notice: Could not automatically update PATH: $_"
        Write-Host "Recommended: Add the following line to your PowerShell `$PROFILE:"
        Write-Host "  `$env:Path = `"$USER_BIN;`$env:Path`""
    }
} else {
    Write-Host "$USER_BIN is already in your PATH."
}

Write-Host "Installation complete."
Write-Host "You can now execute 'zapnano' or 'znano' application."
