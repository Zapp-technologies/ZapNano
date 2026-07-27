#!/bin/bash
set -e

VERSION="0.2.0"
ARCH=$(uname -m)
DIST_NAME="zapnano-$VERSION-macos-$ARCH"
DIST_DIR="dist/$DIST_NAME"

echo "Packaging ZapNano $VERSION for macOS ($ARCH)..."

cd "$(dirname "$0")/.."

if [ ! -f "target/release/zapnano" ]; then
    echo "Building release binary..."
    cargo build --release
fi

echo "Cleaning and preparing dist directory..."
rm -rf "distribution/macOS/dist/"
mkdir -p "distribution/macOS/$DIST_DIR/resources"

echo "Copying binary..."
cp "target/release/zapnano" "distribution/macOS/$DIST_DIR/"

if [ -d "extensions" ]; then
    echo "Copying extensions..."
    cp -r "extensions" "distribution/macOS/$DIST_DIR/resources/"
fi

echo "Copying installer script..."
cp "distribution/macOS/install.sh" "distribution/macOS/$DIST_DIR/"
chmod +x "distribution/macOS/$DIST_DIR/install.sh"

echo "Archiving into .tar.gz..."
cd "distribution/macOS/dist"
tar -czvf "$DIST_NAME.tar.gz" "$DIST_NAME"

echo "Distribution package ready at: distribution/macOS/dist/$DIST_NAME.tar.gz"
