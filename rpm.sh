#!/usr/bin/env bash
set -euo pipefail

BIN_NAME="lz"  # Replace with your actual binary name
PACKAGE_NAME="lz"  # Should match your [package] name in Cargo.toml
COPR_REPO="alexsjones/lz"  # Replace with your COPR username/project

RPMBUILD_DIR="target/release/rpmbuild"
SRPM_DIR="$RPMBUILD_DIR/SRPMS"
OUTPUT_DIR="dist"

echo "🚀 Building release binary..."
cargo build --release

echo "✂️ Stripping debug symbols..."
strip -s "target/release/$BIN_NAME"

echo "📦 Generating SRPM with cargo rpm..."
cargo rpm srpm

# Find the latest generated SRPM
SRPM_FILE=$(find "$SRPM_DIR" -type f -name "${PACKAGE_NAME}-*.src.rpm" | sort | tail -n 1)

if [[ -z "$SRPM_FILE" ]]; then
    echo "❌ No SRPM file found in $SRPM_DIR"
    exit 1
fi

echo "📁 Copying SRPM to $OUTPUT_DIR..."
mkdir -p "$OUTPUT_DIR"
cp -v "$SRPM_FILE" "$OUTPUT_DIR/"

echo "📤 Uploading SRPM to COPR: $COPR_REPO"
copr-cli build "$COPR_REPO" "$SRPM_FILE"

echo "✅ Done! SRPM copied to $OUTPUT_DIR/ and uploaded to COPR."
