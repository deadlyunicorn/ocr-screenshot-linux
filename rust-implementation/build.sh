#!/bin/bash
# Build script for the Rust OCR Screenshot Tool

set -e

echo "🦀 Building Rust OCR Screenshot Tool..."
echo ""

# Source Rust environment
source $HOME/.cargo/env

# Build in release mode
cd "$(dirname "$0")"
cargo build --release

echo ""
echo "✅ Build complete!"
echo ""
echo "Binary location: target/release/ocr-screenshot"
echo ""
echo "To run: ./target/release/ocr-screenshot"
