#!/usr/bin/env bash
set -euo pipefail

PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"
APP_NAME="RingSlint"
BIN_NAME="ring-slint-ios"

# Defaults
BUILD_MODE="release"
TARGET_TYPE="device"  # device | simulator

usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --debug          Build in debug mode (default: release)"
    echo "  --simulator      Build for simulator (default: device)"
    echo "  --install        Install on device via ios-deploy"
    echo "  --run            Install and run on device"
    echo "  -h, --help       Show this help"
    exit 0
}

INSTALL=0
RUN=0

while [[ $# -gt 0 ]]; do
    case "$1" in
        --debug)      BUILD_MODE="debug"; shift ;;
        --simulator)  TARGET_TYPE="simulator"; shift ;;
        --install)    INSTALL=1; shift ;;
        --run)        RUN=1; INSTALL=1; shift ;;
        -h|--help)    usage ;;
        *)            echo "Unknown option: $1"; usage ;;
    esac
done

# Determine target & cargo flags
if [[ "$TARGET_TYPE" == "simulator" ]]; then
    CARGO_TARGET="aarch64-apple-ios-sim"
else
    CARGO_TARGET="aarch64-apple-ios"
fi

CARGO_FLAGS="--target $CARGO_TARGET --bin $BIN_NAME"
if [[ "$BUILD_MODE" == "release" ]]; then
    CARGO_FLAGS="$CARGO_FLAGS --release"
fi

echo "🔨 Building ($BUILD_MODE) for $TARGET_TYPE ($CARGO_TARGET)..."
cd "$PROJECT_DIR"
cargo build $CARGO_FLAGS

# Create .app bundle
BUNDLE_DIR="$PROJECT_DIR/$APP_NAME.app"
rm -rf "$BUNDLE_DIR"
mkdir -p "$BUNDLE_DIR"

# Copy binary
cp "target/$CARGO_TARGET/$BUILD_MODE/$BIN_NAME" "$BUNDLE_DIR/"

# Copy Info.plist
cp "$PROJECT_DIR/Info.plist" "$BUNDLE_DIR/"

# Copy Ring scripts from resources/
if [[ -d "$PROJECT_DIR/resources" ]]; then
    for f in "$PROJECT_DIR/resources/"*.ring; do
        [[ -f "$f" ]] && cp "$f" "$BUNDLE_DIR/"
    done
fi

# Sign
echo "🔏 Signing..."
codesign --force --sign - "$BUNDLE_DIR"

echo "✅ Bundle ready: $BUNDLE_DIR"

# Install
if [[ $INSTALL -eq 1 ]]; then
    if command -v ios-deploy &>/dev/null; then
        echo "📱 Installing on device..."
        if [[ $RUN -eq 1 ]]; then
            ios-deploy --bundle "$BUNDLE_DIR" --debug
        else
            ios-deploy --bundle "$BUNDLE_DIR"
        fi
    else
        echo "⚠️  ios-deploy not found. Install with: brew install ios-deploy"
    fi
fi
