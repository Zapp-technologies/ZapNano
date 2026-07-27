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
rm -rf "installerMacOS/dist/"
mkdir -p "installerMacOS/$DIST_DIR/resources"

echo "Copying binary..."
cp "target/release/zapnano" "installerMacOS/$DIST_DIR/"

echo "Copying resources..."
cp -r "installerWindows/resources/"* "installerMacOS/$DIST_DIR/resources/"
cp -r "extensions" "installerMacOS/$DIST_DIR/resources/"

echo "Copying installer script..."
cp "installerMacOS/install.sh" "installerMacOS/$DIST_DIR/"
chmod +x "installerMacOS/$DIST_DIR/install.sh"

echo "Archiving into .tar.gz..."
cd "installerMacOS/dist"
tar -czvf "$DIST_NAME.tar.gz" "$DIST_NAME"

echo "Distribution package ready at: installerMacOS/dist/$DIST_NAME.tar.gz"
