#!/bin/sh
set -e

echo "Starting Zap Nano installation..."

ZAPNANO_HOME="$HOME/.local/share/ZapNano"
BIN_DIR="$ZAPNANO_HOME/bin"
RES_DIR="$ZAPNANO_HOME/resources"
USER_BIN="$HOME/.local/bin"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"

if [ ! -f "$SCRIPT_DIR/zapnano" ]; then
    echo "Error: 'zapnano' binary not found."
    echo "Please ensure you run this script from the extracted archive directory."
    exit 1
fi

echo "[1/4] Preparing directories..."
mkdir -p "$BIN_DIR"
mkdir -p "$RES_DIR"
mkdir -p "$USER_BIN"

echo "[2/4] Copying files to $ZAPNANO_HOME..."
cp "$SCRIPT_DIR/zapnano" "$BIN_DIR/zapnano"
chmod +x "$BIN_DIR/zapnano"

cat << 'EOF' > "$BIN_DIR/znano"
#!/bin/bash
TARGET_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"
exec "$TARGET_DIR/zapnano" "$@"
EOF
chmod +x "$BIN_DIR/znano"

if [ -d "$SCRIPT_DIR/resources" ]; then
    cp -r "$SCRIPT_DIR/resources/"* "$RES_DIR/" 2>/dev/null || true
fi

echo "[3/4] Creating symlinks in $USER_BIN..."
ln -sf "$BIN_DIR/zapnano" "$USER_BIN/zapnano"
ln -sf "$BIN_DIR/znano" "$USER_BIN/znano"

echo "[4/4] Finalizing Paths..."
if [[ ":$PATH:" != *":$USER_BIN:"* ]]; then
    echo "Notice: $USER_BIN is not in your PATH."
    echo "Recommended: Add the following line to your ~/.bashrc or ~/.zshrc:"
    echo "export PATH=\"\$HOME/.local/bin:\$PATH\""
fi

echo "Installation complete."
echo "You can now execute 'zapnano' or 'znano' application."
