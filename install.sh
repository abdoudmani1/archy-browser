#!/bin/bash

# Exit immediately if any command fails
set -e

echo "========================================"
echo "  Archy Browser Installer (Arch Linux)  "
echo "========================================"
echo ""

echo "[1/3] Installing required Arch Linux packages..."
# --needed prevents reinstalling packages if they are already present
echo "[1/3] System Dependencies"
read -p "Do you want to install the required Arch packages ? [y/N] " -n 1 -r
echo ""

if [[ $REPLY =~ ^[Yy]$ ]]
then
    echo "Installing packages..."
    sudo pacman -S --needed base-devel rust gtk3 webkit2gtk-4.1 gst-plugins-base gst-plugins-good gst-plugins-bad gst-plugins-ugly gst-libav
else
    echo "Skipping package installation. (Ensure you have them installed manually or the build will fail!)"
fi

echo ""
echo "[2/3] Compiling Archy Browser from source..."
# Build the project for maximum performance
env RUSTFLAGS="-C target-cpu=native" cargo build --release

echo "==> Installing binary..."
sudo cp target/release/archy-browser /usr/local/bin/archy-browser
sudo chmod 755 /usr/local/bin/archy-browser

echo "==> Installing Wayland Desktop Integration..."
# Install the .desktop file
mkdir -p ~/.local/share/applications
cp com.system.archy.desktop ~/.local/share/applications/

# Install the custom logo (Assumes a .png, update to .svg if you used vector)
mkdir -p ~/.local/share/icons/hicolor/256x256/apps/
cp archy-logo.png ~/.local/share/icons/hicolor/256x256/apps/archy-logo.png

# Refresh the system icon cache so Hyprland detects it immediately
gtk-update-icon-cache -f -t ~/.local/share/icons/hicolor/ || true

echo ""
echo "[3/3] Installing to your system..."
# Copy the binary to a global directory so it can be run from the terminal
# (Change 'archy-browser' to 'argon-browser' in the path below if you haven't updated your Cargo.toml name on this branch)
sudo cp target/release/archy-browser /usr/local/bin/archy-browser
sudo chmod +x /usr/local/bin/archy-browser

echo ""
echo "✅ Installation complete! You can now launch the browser by typing 'archy-browser' in your terminal."
