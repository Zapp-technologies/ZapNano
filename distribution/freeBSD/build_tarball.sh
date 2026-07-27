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
rm -rf "distribution/freeBSD/dist/"
mkdir -p "distribution/freeBSD/$DIST_DIR/resources"

echo "Copying binary..."
cp "$BINARY" "distribution/freeBSD/$DIST_DIR/"

if [ -d "extensions" ]; then
    echo "Copying extensions..."
    cp -r "extensions" "distribution/freeBSD/$DIST_DIR/resources/"
fi

echo "Copying installer script..."
cp "distribution/freeBSD/install.sh" "distribution/freeBSD/$DIST_DIR/"
chmod +x "distribution/freeBSD/$DIST_DIR/install.sh"

echo "Archiving into .tar.gz..."
cd "distribution/freeBSD/dist"
tar -czvf "$DIST_NAME.tar.gz" "$DIST_NAME"

echo "Distribution package ready at: distribution/freeBSD/dist/$DIST_NAME.tar.gz"
