#  Archy Browser

An ultra-minimalist, insanely lightweight Wayland web browser built in Rust and WebKit2GTK. 

Archy is designed to be fast, stay out of your way, and aggressively optimize its own memory usage based on your system's hardware. Whether you are running on a 4GB potato or a 16GB workstation, Archy automatically scales its background processes, media handling, and caching to give you the best possible browsing experience without hoarding your RAM.

![GitHub Release](https://img.shields.io/github/v/release/abdoudmani1/archy-browser?style=flat-square)
![Rust](https://img.shields.io/badge/Language-Rust-orange?style=flat-square)


---

##  Features

- **Dynamic RAM Auto-Scaling:** Archy detects your total system memory on startup and applies one of three strict profiles:
  - **4GB Profile (Extreme):** Disables background media, kills page caching, and triggers aggressive memory flushing the exact millisecond you switch tabs.
  - **8GB Profile (Balanced):** Allows background audio for music streaming, but keeps memory limits tight.
  - **16GB+ Profile (Performance):** Enables full history caching for instant back/forward navigation and smooth multitasking.
- **Native Wayland & Hardware Acceleration:** GPU-accelerated rendering by default for 60FPS video playback and smooth scrolling.
- **Memory Defragmentation:** Powered by `tikv-jemallocator` to prevent long-term memory leaks and fragmentation.
- **Catppuccin Mocha UI:** A gorgeous, built-in start page and clean navigation bar that doesn't distract you from the web.
- **Built-in Protocol:** Type `archy://settings` to view real-time system telemetry and your active memory profile, or `archy://start` to return home.

##  Installation

### Option 1: Universal AppImage (All Linux Distros)
You can run Archy on Ubuntu, Fedora, Mint, and Debian without compiling anything.
1. Go to the [Releases page](https://github.com/abdoudmani1/archy-browser/releases).
2. Download the latest `Archy-x86_64.AppImage`.
3. Right-click the file, go to **Properties > Permissions**, and check **"Allow executing file as program"**.
4. Double-click to run!

### Option 2: Native Install (Arch Linux)
For the best performance and native desktop integration on Arch Linux/Hyprland:
```bash
git clone [https://github.com/abdoudmani1/archy-browser.git](https://github.com/abdoudmani1/archy-browser.git)
cd archy-browser
chmod +x install.sh
./install.sh
```

## Building from Source

If you want to modify the browser or build it manually on another distro:

**1. Install System Dependencies**
- **Arch Linux:** `sudo pacman -S base-devel rust gtk3 webkit2gtk-4.1 gst-plugins-good gst-plugins-bad gst-libav`
- **Ubuntu/Debian:** `sudo apt install build-essential rustc cargo libgtk-3-dev libwebkit2gtk-4.1-dev`

**2. Compile and Run**
```bash
cargo build --release
./target/release/archy-browser
```

## Shortcuts & Navigation
- **`+` Button:** Opens a new tab instantly.
- **`X` Button:** Closes the tab (and flushes RAM instantly on low-spec systems).
- **`⚙` Button:** Opens your internal hardware and memory settings.
- **Smart Search:** Typing a query (like `rust documentation`) automatically searches Google, while typing a domain (like `github.com`) navigates directly to the site.

## Contributing
Pull requests are welcome! If you want to help make Archy even lighter or add new features (like ad-blocking), feel free to open an issue or submit a PR.

## License
This project is open-source and available under the MIT License.
