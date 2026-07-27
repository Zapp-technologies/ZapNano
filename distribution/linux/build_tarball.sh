#!/bin/bash
set -e

VERSION="0.2.0"
DIST_NAME="zapnano-$VERSION-linux-x86_64"
DIST_DIR="dist/$DIST_NAME"

echo "Packaging ZapNano $VERSION for Linux..."

cd "$(dirname "$0")/../.."

# Check if we built in the standard release path or cross linux path
if [ -f "target/aarch64-unknown-linux-gnu/release/zapnano" ]; then
    BINARY="target/aarch64-unknown-linux-gnu/release/zapnano"
elif [ -f "target/release/zapnano" ]; then
    BINARY="target/release/zapnano"
else
    echo "Building release binary..."
    cargo build --release
    BINARY="target/release/zapnano"
fi

echo "Cleaning and preparing dist directory..."
rm -rf "distribution/linux/dist/"
mkdir -p "$DIST_DIR/resources"

echo "Copying binary..."
cp "$BINARY" "$DIST_DIR/"

if [ -d "extensions" ]; then
    echo "Copying extensions..."
    cp -r "extensions" "$DIST_DIR/resources/"
fi

echo "Copying installer script..."
cp "distribution/linux/install.sh" "$DIST_DIR/"
chmod +x "$DIST_DIR/install.sh"

echo "Archiving into .tar.gz..."
cd "distribution/linux/dist"
tar -czvf "$DIST_NAME.tar.gz" "$DIST_NAME"

echo "Distribution package ready at: distribution/linux/dist/$DIST_NAME.tar.gz"
