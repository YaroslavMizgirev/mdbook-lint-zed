#!/bin/bash
set -e

echo "🔨 Building mdbook-lint Zed extension for local testing..."

# Build the extension
echo "Building WebAssembly..."
cargo build --target wasm32-wasip1 --release

# Create local extension directory
EXT_DIR="$HOME/.config/zed/extensions/mdbook-lint"
echo "Creating extension directory: $EXT_DIR"
mkdir -p "$EXT_DIR"

# Copy files
echo "Copying extension files..."
cp target/wasm32-wasip1/release/mdbook_lint_zed.wasm "$EXT_DIR/"
cp extension.toml "$EXT_DIR/"

echo "✅ Extension installed locally!"
echo ""
echo "Next steps:"
echo "1. Restart Zed editor"
echo "2. Open a markdown file or mdBook project"
echo "3. Check for mdbook-lint diagnostics"
echo ""
echo "To test with violations, create a file like:"
echo "echo '# Level 1' > test.md"
echo "echo '### Level 3 - skipped level 2' >> test.md"
echo "echo '' >> test.md"
echo "echo '```' >> test.md"
echo "echo 'code without language tag' >> test.md"
echo "echo '```' >> test.md"
echo ""
echo "Then open test.md in Zed - you should see red/yellow underlines!"
echo ""
echo "To uninstall: rm -rf '$EXT_DIR'"