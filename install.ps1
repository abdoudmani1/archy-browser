$ErrorActionPreference = "Stop"

Write-Host "========================================"
Write-Host "   Archy Browser Installer (Windows)    "
Write-Host "========================================"

Write-Host "`[1/2] Compiling Archy Browser (release)..."
$env:RUSTFLAGS = "-C target-cpu=native"
cargo build --release

$InstallDir = "$env:LOCALAPPDATA\ArchyBrowser"
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Copy-Item "target\release\archy-browser.exe" "$InstallDir\archy-browser.exe" -Force

Write-Host "[2/2] Creating Start Menu & Desktop shortcuts..."
$WshShell = New-Object -ComObject WScript.Shell
$Paths = @(
    "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Archy Browser.lnk",
    "$env:USERPROFILE\Desktop\Archy Browser.lnk"
)

foreach ($ShortcutPath in $Paths) {
    $Shortcut = $WshShell.CreateShortcut($ShortcutPath)
    $Shortcut.TargetPath = "$InstallDir\archy-browser.exe"
    $Shortcut.WorkingDirectory = $InstallDir
    $Shortcut.Description = "Ultra-minimalist web browser"
    $Shortcut.Save()
}

Write-Host "`Installation complete! Launch 'Archy Browser' from your Desktop or Start Menu."$ErrorActionPreference = "Stop"

Write-Host "========================================"
Write-Host "   Archy Browser Installer (Windows)    "
Write-Host "========================================"

Write-Host "`n[1/2] Compiling Archy Browser (release)..."
$env:RUSTFLAGS = "-C target-cpu=native"
cargo build --release

$InstallDir = "$env:LOCALAPPDATA\ArchyBrowser"
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Copy-Item "target\release\archy-browser.exe" "$InstallDir\archy-browser.exe" -Force

Write-Host "[2/2] Creating Start Menu & Desktop shortcuts..."
$WshShell = New-Object -ComObject WScript.Shell
$Paths = @(
    "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Archy Browser.lnk",
    "$env:USERPROFILE\Desktop\Archy Browser.lnk"
)

foreach ($ShortcutPath in $Paths) {
    $Shortcut = $WshShell.CreateShortcut($ShortcutPath)
    $Shortcut.TargetPath = "$InstallDir\archy-browser.exe"
    $Shortcut.WorkingDirectory = $InstallDir
    $Shortcut.Description = "Ultra-minimalist web browser"
    $Shortcut.Save()
}

Write-Host "`nInstallation complete! Launch 'Archy Browser' from your Desktop or Start Menu."