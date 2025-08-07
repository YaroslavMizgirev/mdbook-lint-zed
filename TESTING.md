# Testing the mdbook-lint Zed Extension

This guide covers how to test the extension locally before using or contributing to it.

## Prerequisites

1. **Install mdbook-lint**: `cargo install mdbook-lint`
2. **Install Zed**: Download from [zed.dev](https://zed.dev/)
3. **Verify mdbook-lint works**: `mdbook-lint --version`

## Quick Local Installation

Run the provided test script:

```bash
./test-local.sh
```

This will:
- Build the WebAssembly extension
- Install it to your Zed extensions directory (`~/.config/zed/extensions/mdbook-lint/`)
- Provide next steps for testing

## Manual Testing Steps

### 1. Test Basic Markdown Linting

Open `test-violations.md` in Zed. You should see:

- **Red underlines** for errors:
  - MD001: Heading level skip (# Level 1 → ### Level 3)
  - MD024: Duplicate headings
  
- **Yellow underlines** for warnings:
  - MD013: Line too long
  - MD040: Missing code language
  - MD009: Trailing spaces
  - MD027: Blockquote inconsistent spacing
  - MD030: List spacing issues

### 2. Test mdBook Project Features

1. Open the `test-mdbook/` directory in Zed
2. The extension should detect it's an mdBook project
3. Open `test-mdbook/src/introduction.md` and look for:
   - MDBOOK001: Code blocks without language tags
   - MDBOOK002: Links to missing files
   - MDBOOK004: Duplicate headings across files

### 3. Test Configuration

Create a config file in the test directory:

```bash
cat > test-mdbook/mdbook-lint.toml << EOF
[core]
# Disable line length rule for testing
disabled_rules = ["MD013"]

# Only show errors, not warnings
fail_on_warnings = false
EOF
```

Restart Zed and notice MD013 violations should disappear.

### 4. Test LSP Server Communication

Check Zed's output panel for LSP messages:
1. Open Zed
2. Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux)
3. Type "Output" and select it
4. Look for mdbook-lint LSP server startup messages

### 5. Test Error Handling

Test what happens when mdbook-lint isn't installed:
1. Temporarily rename mdbook-lint: `mv ~/.cargo/bin/mdbook-lint ~/.cargo/bin/mdbook-lint.bak`
2. Restart Zed
3. Open a markdown file
4. Check the output panel for error messages
5. Restore: `mv ~/.cargo/bin/mdbook-lint.bak ~/.cargo/bin/mdbook-lint`

## Expected Behavior

### ✅ Working Correctly

- Diagnostics appear as you type or save files
- Red underlines for errors, yellow for warnings, blue for info
- Hover over violations shows detailed explanations
- mdBook projects get enhanced rule checking
- Configuration files are automatically discovered

### ❌ Common Issues

**No diagnostics appear:**
- Check mdbook-lint is installed: `which mdbook-lint`
- Check Zed output panel for LSP errors
- Ensure file has `.md` extension
- Try saving the file (LSP may activate on save)

**Extension not loading:**
- Check Zed logs for WebAssembly errors
- Verify extension files exist: `ls ~/.config/zed/extensions/mdbook-lint/`
- Try rebuilding: `./test-local.sh`

**LSP server crashes:**
- Check mdbook-lint works directly: `mdbook-lint lint test-violations.md`
- Look for error messages in Zed output panel
- Try running LSP server manually: `mdbook-lint lsp --stdio`

## Performance Testing

Test with larger files:
1. Create a large markdown file: `cat test-violations.md test-violations.md > large-test.md` (repeat several times)
2. Open in Zed and check response time
3. Verify diagnostics still appear correctly

## Debugging

### Enable Debug Logging

If you need more detailed information:
1. Set environment variable: `export RUST_LOG=debug`
2. Restart Zed
3. Check output panel for detailed logs

### Manual LSP Server Testing

Test the LSP server independently:

```bash
# Start server
mdbook-lint lsp --stdio

# Or with TCP for easier debugging
mdbook-lint lsp --port 8080
```

### WebAssembly Issues

If the extension fails to load:
1. Check build was successful: `ls -la target/wasm32-wasip1/release/mdbook_lint_zed.wasm`
2. Verify file was copied: `ls -la ~/.config/zed/extensions/mdbook-lint/`
3. Check Zed console for WebAssembly errors

## Clean Up

To remove the test installation:

```bash
rm -rf ~/.config/zed/extensions/mdbook-lint
```

Then restart Zed.

## Automated Testing

The extension includes CI/CD that tests:
- Code formatting (`cargo fmt`)
- Linting (`cargo clippy`) 
- WebAssembly compilation
- Cross-platform builds

Run these locally:
```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo build --target wasm32-wasip1 --release
```

## Reporting Issues

When reporting bugs, please include:
1. Zed version: Help → About Zed
2. Operating system and version
3. mdbook-lint version: `mdbook-lint --version`
4. Steps to reproduce
5. Expected vs actual behavior
6. Zed output panel logs
7. Sample markdown that triggers the issue

## Next Steps

After successful local testing:
1. The extension can be submitted to Zed's extension registry
2. Create usage documentation with screenshots
3. Consider additional features based on testing feedback