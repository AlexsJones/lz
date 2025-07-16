#!/usr/bin/env bash
set -euo pipefail

BIN_NAME="lz"
TARGET_DIR="target/release"
RPM_OUTPUT_DIR="target/generate-rpm"
OUTPUT_DIR="dist"

echo "🚀 Building release binary..."
cargo build --release

echo "✂️ Stripping debug symbols..."
strip -s "${TARGET_DIR}/${BIN_NAME}"

echo "📦 Generating RPM..."
cargo generate-rpm

# Find the actual RPM file (first one that matches the binary name)
RPM_FILE=$(find "${RPM_OUTPUT_DIR}" -type f -name "${BIN_NAME}-*.rpm" | head -n 1)

if [[ -z "$RPM_FILE" ]]; then
    echo "❌ RPM file not found in ${RPM_OUTPUT_DIR}"
    exit 1
fi

echo "📁 Moving RPM to ${OUTPUT_DIR}/..."
mkdir -p "${OUTPUT_DIR}"
cp -v "$RPM_FILE" "${OUTPUT_DIR}/"

echo "✅ Done! RPM is in ${OUTPUT_DIR}/"
