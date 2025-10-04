# OCR Screenshot Linux

A fast, native Linux application built with Rust and GTK4 that allows you to paste images from your clipboard and extract text using Tesseract OCR.

## Features

- 📋 **Native Clipboard Support**: Press Ctrl+V to paste images directly from clipboard
- 🖼️ **Image Preview**: View the pasted image before OCR
- 📝 **Text Extraction**: Automatic OCR using Tesseract with optimized preprocessing
- ✂️ **Selectable Text**: Extracted text is fully selectable and copyable
- 🎨 **Native GTK4 Interface**: Beautiful interface that matches your system theme
- ⚡ **Fast Performance**: Built with Rust for optimal speed and memory efficiency
- 🔧 **Advanced Preprocessing**: CLAHE enhancement, Gaussian blur, Otsu thresholding, and upscaling
- 🚀 **Easy to Use**: Simple and intuitive interface

## Requirements

- Rust 1.70 or higher
- GTK 4.0
- Tesseract OCR 4.0+
- Linux (tested on Ubuntu/Debian, but should work on other distributions)

## Installation

### 1. Install System Dependencies

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install libgtk-4-dev libtesseract-dev libleptonica-dev clang libclang-dev pkg-config
```

**Fedora:**
```bash
sudo dnf install gtk4-devel tesseract-devel leptonica-devel clang clang-devel
```

**Arch Linux:**
```bash
sudo pacman -S gtk4 tesseract leptonica clang
```

### 2. Install Rust

If you don't have Rust installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 3. Build the Application

```bash
cd rust-implementation
cargo build --release
```

The compiled binary will be at `target/release/ocr-screenshot`

## Usage

### Running the Application

```bash
cd rust-implementation
./target/release/ocr-screenshot
```

Or use the build script:
```bash
cd rust-implementation
./build.sh
```

### Using the Application

1. **Take a screenshot or copy an image**:
   - Use your system's screenshot tool (e.g., `gnome-screenshot`, `spectacle`, `flameshot`)
   - Or copy any image to your clipboard

2. **Paste into the app**:
   - Click on the app window to focus it
   - Press `Ctrl+V`

3. **View results**:
   - The image will be displayed
   - OCR will run automatically in the background
   - Select and copy the extracted text

### Keyboard Shortcuts

- `Ctrl+V`: Paste image from clipboard
- `Ctrl+C`: Copy selected text from results

## Technical Details

### Image Preprocessing Pipeline

The application uses a sophisticated preprocessing pipeline to improve OCR accuracy:

1. **Grayscale Conversion**: Converts image to grayscale
2. **CLAHE Enhancement**: Contrast Limited Adaptive Histogram Equalization for better contrast
3. **Gaussian Blur**: Reduces noise (5x5 kernel)
4. **Otsu Thresholding**: Adaptive binary thresholding for optimal text separation
5. **Upscaling**: Scales images up to minimum 1500px for better recognition
6. **Lanczos Filtering**: High-quality resampling during upscaling

### Tesseract Configuration

- **OCR Engine Mode**: LSTM neural network (mode 1)
- **Page Segmentation Mode**: Single column text (mode 4)
- **Post-processing**: Converts common OCR errors (e.g., `{` to `(`)

## Troubleshooting

### Build Errors

**"Could not find Tesseract":**
```bash
# Verify tesseract is installed
tesseract --version

# Check pkg-config can find it
pkg-config --modversion tesseract
```

**"libclang shared library is not loaded":**
```bash
# Install clang development packages
sudo apt-get install clang libclang-dev  # Ubuntu/Debian
```

### Runtime Issues

**No image in clipboard:**
- Make sure you've copied an image to your clipboard
- Most screenshot tools have a "Copy to clipboard" option

**OCR not working:**
```bash
# Verify Tesseract is installed and has English language data
tesseract --list-langs
```

## Development

### Project Structure

```
ocr-screenshot-linux/
├── rust-implementation/
│   ├── src/
│   │   └── main.rs          # Main application code
│   ├── Cargo.toml            # Rust dependencies
│   ├── build.sh              # Build script
│   └── README.md             # Implementation details
├── test-image.png            # Test image for validation
└── README.md                 # This file
```

### Building in Debug Mode

```bash
cd rust-implementation
cargo build
./target/debug/ocr-screenshot
```

## Legacy Python Implementation

A Python/GTK3 implementation is available in the repository root for reference. The Rust implementation is recommended for production use due to better performance and reliability.

## License

MIT License

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

---

**Built with assistance from Claude (Sonnet 3.5)** 🤖✨
