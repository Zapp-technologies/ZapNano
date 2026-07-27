#!/bin/bash
set -e

VERSION="0.2.0"
DIST_NAME="zapnano-$VERSION-illumos-x86_64"
DIST_DIR="distribution/illumos/dist/$DIST_NAME"

echo "Packaging ZapNano $VERSION for Illumos..."

cd "$(dirname "$0")/../.."

if [ -f "target/x86_64-unknown-illumos/release/zapnano" ]; then
    BINARY="target/x86_64-unknown-illumos/release/zapnano"
elif [ -f "target/release/zapnano" ]; then
    BINARY="target/release/zapnano"
else
    echo "Warning: target/release/zapnano not found. Running cargo build..."
    cargo build --release
    BINARY="target/release/zapnano"
fi

echo "Cleaning and preparing dist directory..."
rm -rf "distribution/illumos/dist/"
mkdir -p "$DIST_DIR/resources"

echo "Copying binary..."
cp "$BINARY" "$DIST_DIR/"

if [ -d "extensions" ]; then
    echo "Copying extensions..."
    cp -r "extensions" "$DIST_DIR/resources/"
fi

echo "Copying installer script..."
cp "distribution/illumos/install.sh" "$DIST_DIR/"
chmod +x "$DIST_DIR/install.sh"

echo "Creating tarball..."
cd distribution/illumos/dist
tar -czvf "$DIST_NAME.tar.gz" "$DIST_NAME"

echo "Distribution package ready at: distribution/illumos/dist/$DIST_NAME.tar.gz"
