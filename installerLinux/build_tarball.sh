#!/bin/bash
set -e

VERSION="0.2.0"
DIST_NAME="zapnano-$VERSION-linux-x86_64"
DIST_DIR="dist/$DIST_NAME"

echo "Packaging ZapNano $VERSION for Linux..."

cd "$(dirname "$0")/.."

if [ ! -f "target/release/zapnano" ]; then
    echo "Building release binary..."
    cargo build --release
fi

echo "Cleaning and preparing dist directory..."
rm -rf "installerLinux/dist/"
mkdir -p "installerLinux/$DIST_DIR/resources"

echo "Copying binary..."
cp "target/release/zapnano" "installerLinux/$DIST_DIR/"

echo "Copying resources..."
cp -r "installerWindows/resources/"* "installerLinux/$DIST_DIR/resources/"
cp -r "extensions" "installerLinux/$DIST_DIR/resources/"

echo "Copying installer script..."
cp "installerLinux/install.sh" "installerLinux/$DIST_DIR/"
chmod +x "installerLinux/$DIST_DIR/install.sh"

echo "Archiving into .tar.gz..."
cd "installerLinux/dist"
tar -czvf "$DIST_NAME.tar.gz" "$DIST_NAME"

echo "Distribution package ready at: installerLinux/dist/$DIST_NAME.tar.gz"
