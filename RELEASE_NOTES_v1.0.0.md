# OCR Screenshot Linux v1.0.0 🎉

**First stable release!** A fast, native Linux OCR application built with Rust and GTK4.

## 🚀 What's New

This is the initial release of OCR Screenshot Linux, featuring:

- ✨ **Fast Native Application**: Built with Rust for optimal performance
- 📋 **Clipboard Integration**: Paste images directly with Ctrl+V
- 🔍 **Advanced OCR**: Tesseract integration with optimized preprocessing
- 🎨 **Modern UI**: GTK4 interface that matches your system theme
- ⚡ **Async Processing**: Non-blocking OCR with channel-based communication
- 🛠️ **Smart Preprocessing**: CLAHE, Otsu thresholding, and intelligent upscaling

## 📦 Downloads

### ARM64 / aarch64 (Pre-built Binary)

**File**: `ocr-screenshot-linux-v1.0.0-aarch64.tar.gz` (339 KB compressed, 810 KB binary)

**Architecture**: ARM 64-bit (aarch64) - for Raspberry Pi 4/5, Apple Silicon under Linux, ARM servers, etc.

```bash
# Quick start
wget https://github.com/deadlyunicorn/ocr-screenshot-linux/releases/download/v1.0.0/ocr-screenshot-linux-v1.0.0-aarch64.tar.gz
tar -xzf ocr-screenshot-linux-v1.0.0-aarch64.tar.gz
cd release-package
./ocr-screenshot
```

### Other Architectures (Build from Source)

For x86_64 (Intel/AMD) or other architectures:

```bash
git clone https://github.com/deadlyunicorn/ocr-screenshot-linux.git
cd ocr-screenshot-linux/rust-implementation
cargo build --release
```

## 📋 System Requirements

### Runtime Dependencies

**Ubuntu/Debian:**
```bash
sudo apt-get install libgtk-4-1 tesseract-ocr tesseract-ocr-eng
```

**Fedora:**
```bash
sudo dnf install gtk4 tesseract tesseract-langpack-eng
```

**Arch Linux:**
```bash
sudo pacman -S gtk4 tesseract tesseract-data-eng
```

### Build Dependencies (for source builds)

See the [README](https://github.com/deadlyunicorn/ocr-screenshot-linux#installation) for complete build instructions.

## 🎯 Features

### Core Functionality
- **Clipboard Support**: Press Ctrl+V to paste images from clipboard
- **Auto OCR**: Automatically processes images upon paste
- **Text Selection**: Extract and copy recognized text
- **Image Preview**: View pasted images before OCR

### Technical Highlights
- **Language**: Rust 🦀
- **GUI Framework**: GTK4
- **OCR Engine**: Tesseract 5.x
- **Image Processing**: Custom preprocessing pipeline
  - Grayscale conversion
  - CLAHE (Contrast Limited Adaptive Histogram Equalization)
  - Otsu thresholding for optimal binarization
  - Smart upscaling for small images (min 1500px)
  - Lanczos3 filtering for quality resampling

### Performance
- **Binary Size**: 810 KB (uncompressed)
- **Memory Usage**: ~10-20 MB RAM
- **Startup Time**: Sub-second
- **OCR Speed**: 1-3 seconds typical

## 📖 Usage

1. **Launch the application**
2. **Copy an image** (screenshot, file, or browser)
3. **Press Ctrl+V** in the app window
4. **Copy the text** from the results area

### Keyboard Shortcuts
- `Ctrl+V`: Paste image from clipboard
- `Ctrl+C`: Copy extracted text

## 🔧 Installation

### Quick Install (ARM64)

```bash
# Download and extract
tar -xzf ocr-screenshot-linux-v1.0.0-aarch64.tar.gz
cd release-package

# Install to user bin
mkdir -p ~/.local/bin
cp ocr-screenshot ~/.local/bin/
chmod +x ~/.local/bin/ocr-screenshot

# Add to PATH (add to ~/.bashrc)
export PATH="$HOME/.local/bin:$PATH"

# Run
ocr-screenshot
```

### Build from Source

```bash
# Clone repository
git clone https://github.com/deadlyunicorn/ocr-screenshot-linux.git
cd ocr-screenshot-linux/rust-implementation

# Install Rust if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Build
cargo build --release

# Binary location
./target/release/ocr-screenshot
```

## 🐛 Known Issues

- Currently supports English language only (easily extendable)
- Requires X11/Wayland display server
- Best results with clear, high-contrast text

## 🔮 Future Plans

- [ ] Multi-language support
- [ ] Drag-and-drop file support
- [ ] OCR history
- [ ] System tray integration
- [ ] Custom preprocessing profiles
- [ ] Batch processing

## 🤝 Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues.

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details

## 🙏 Acknowledgments

- [Tesseract OCR](https://github.com/tesseract-ocr/tesseract) - Powerful OCR engine
- [GTK](https://www.gtk.org/) - GUI toolkit
- [Rust GTK4 bindings](https://gtk-rs.org/) - Excellent Rust bindings

## 📊 Technical Details

### Dependencies
```toml
gtk4 = "0.9"
gdk4 = "0.9"
glib = "0.20"
image = "0.25"
tesseract = "0.15"
imageproc = "0.25"
```

### Compilation Options
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

### Architecture Details
- **Platform**: Linux (ARM64/aarch64)
- **Format**: ELF 64-bit LSB pie executable
- **Linking**: Dynamically linked
- **Stripped**: No (debug symbols included)

## 💬 Support

- **Issues**: [GitHub Issues](https://github.com/deadlyunicorn/ocr-screenshot-linux/issues)
- **Discussions**: [GitHub Discussions](https://github.com/deadlyunicorn/ocr-screenshot-linux/discussions)

---

**Built with assistance from Claude (Sonnet 3.5)** 🤖✨

**Release Date**: October 4, 2025