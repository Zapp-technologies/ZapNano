#!/bin/bash
set -e

VERSION="0.2.0"
DIST_NAME="zapnano-$VERSION-freebsd-x86_64"
DIST_DIR="dist/$DIST_NAME"

echo "Packaging ZapNano $VERSION for FreeBSD..."

cd "$(dirname "$0")/.."

# Check if we built in the standard release path or cross freebsd path
if [ -f "target/x86_64-unknown-freebsd/release/zapnano" ]; then
    BINARY="target/x86_64-unknown-freebsd/release/zapnano"
elif [ -f "target/release/zapnano" ]; then
    BINARY="target/release/zapnano"
else
    echo "Building release binary..."
    cargo build --release
    BINARY="target/release/zapnano"
fi

echo "Cleaning and preparing dist directory..."
rm -rf "installerFreeBSD/dist/"
mkdir -p "installerFreeBSD/$DIST_DIR/resources"

echo "Copying binary..."
cp "$BINARY" "installerFreeBSD/$DIST_DIR/"

echo "Copying resources..."
cp -r "installerWindows/resources/"* "installerFreeBSD/$DIST_DIR/resources/"
cp -r "extensions" "installerFreeBSD/$DIST_DIR/resources/"

echo "Copying installer script..."
cp "installerFreeBSD/install.sh" "installerFreeBSD/$DIST_DIR/"
chmod +x "installerFreeBSD/$DIST_DIR/install.sh"

echo "Archiving into .tar.gz..."
cd "installerFreeBSD/dist"
tar -czvf "$DIST_NAME.tar.gz" "$DIST_NAME"

echo "Distribution package ready at: installerFreeBSD/dist/$DIST_NAME.tar.gz"
