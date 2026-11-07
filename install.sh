#!/usr/bin/env bash
set -e

echo "==> Compiling Argon Browser..."
env RUSTFLAGS="-C target-cpu=native" cargo build --release

echo "==> Installing binary..."
sudo cp target/release/argon-browser /usr/local/bin/argon-browser
sudo chmod 755 /usr/local/bin/argon-browser

echo "==> Installing Wayland Desktop Integration..."
# Install the .desktop file
mkdir -p ~/.local/share/applications
cp com.system.argon.desktop ~/.local/share/applications/

# Install the custom logo (Assumes a .png, update to .svg if you used vector)
mkdir -p ~/.local/share/icons/hicolor/256x256/apps/
cp argon-logo.png ~/.local/share/icons/hicolor/256x256/apps/argon-logo.png

# Refresh the system icon cache so Hyprland detects it immediately
gtk-update-icon-cache -f -t ~/.local/share/icons/hicolor/ || true

echo "==> Success! Argon now supports tabs and proper Wayland branding."