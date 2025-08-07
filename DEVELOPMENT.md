# Development Guide

This guide covers how to develop and test the mdbook-lint Zed extension.

## Prerequisites

1. **Rust**: Install via [rustup](https://rustup.rs/)
2. **WebAssembly target**: `rustup target add wasm32-wasip1`
3. **mdbook-lint**: `cargo install mdbook-lint`
4. **Zed editor**: Download from [zed.dev](https://zed.dev/)

## Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/joshrotenberg/mdbook-lint-zed
   cd mdbook-lint-zed
   ```

2. Build the extension:
   ```bash
   cargo build --target wasm32-wasip1
   ```

3. The WebAssembly binary will be created at:
   ```
   target/wasm32-wasip1/debug/mdbook_lint_zed.wasm
   ```

## Testing Locally

### Method 1: Development Installation

1. Build the extension:
   ```bash
   cargo build --target wasm32-wasip1 --release
   ```

2. Create a local extensions directory structure:
   ```bash
   mkdir -p ~/.config/zed/extensions/mdbook-lint
   cp target/wasm32-wasip1/release/mdbook_lint_zed.wasm ~/.config/zed/extensions/mdbook-lint/
   cp extension.toml ~/.config/zed/extensions/mdbook-lint/
   ```

3. Restart Zed to load the extension

### Method 2: Development Mode

Zed supports loading extensions in development mode. Check the Zed documentation for the latest instructions on development mode.

## Testing the Extension

1. Create a test markdown file:
   ```bash
   echo "# Level 1\n### Level 3 - skipped level 2\n\nSome **bold text** here.\n\n\`\`\`\ncode without language\n\`\`\`" > test.md
   ```

2. Open the file in Zed
3. You should see:
   - Red underlines for errors (MD001: skipped heading level)
   - Yellow underlines for warnings (MD040: missing code language)
   - Hover over violations to see detailed explanations

## Testing with mdBook Projects

1. Create a test mdBook project:
   ```bash
   mkdir test-book && cd test-book
   echo '[book]\ntitle = "Test Book"' > book.toml
   echo '# Summary\n\n- [Chapter 1](chapter1.md)' > SUMMARY.md
   echo '# Chapter 1\n\nSome content.' > chapter1.md
   ```

2. Open the project in Zed
3. The extension should detect it as an mdBook project and enable additional rules

## LSP Server Testing

The extension communicates with the mdbook-lint LSP server. You can test the server directly:

```bash
# Test LSP server
mdbook-lint lsp --stdio

# Test with TCP (useful for debugging)
mdbook-lint lsp --port 8080
```

## Debugging

### Zed Logs

View Zed extension logs:
1. Open Zed
2. Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux)
3. Type "Open Log" and select it
4. Look for mdbook-lint related messages

### LSP Server Logs

The LSP server logs to stderr. You can capture logs by running:

```bash
mdbook-lint lsp --stdio 2> lsp.log
```

### Extension Build Issues

Common build issues and solutions:

1. **Missing WebAssembly target**:
   ```bash
   rustup target add wasm32-wasip1
   ```

2. **API version mismatch**:
   - Check the latest `zed_extension_api` version
   - Update `Cargo.toml` accordingly

3. **Missing mdbook-lint binary**:
   ```bash
   cargo install mdbook-lint
   ```

## Code Structure

```
src/
├── lib.rs              # Main extension implementation
│   ├── MdBookLintExtension  # Main extension struct  
│   ├── language_server_command          # LSP server startup
│   ├── language_server_initialization_options  # LSP init config
│   ├── language_server_workspace_configuration # LSP workspace config
│   └── language_server_binary_path     # Binary path resolution
```

### Key Components

- **Extension Registration**: `register_extension!` macro
- **LSP Integration**: Implements LSP server command and configuration
- **Project Detection**: Automatically detects mdBook projects
- **Binary Resolution**: Finds mdbook-lint binary in various locations

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes
4. Test the extension thoroughly
5. Submit a pull request

### Code Style

- Follow standard Rust formatting: `cargo fmt`
- Run clippy: `cargo clippy`
- Ensure WebAssembly build passes: `cargo build --target wasm32-wasip1`

## Publishing

The extension will be published to the Zed extensions registry. The process involves:

1. Fork `zed-industries/extensions`
2. Add this extension as a Git submodule
3. Update the extensions registry
4. Open a pull request

See the [Zed extensions documentation](https://zed.dev/docs/extensions/developing-extensions) for detailed publishing instructions.

## Troubleshooting

### Extension Not Loading

1. Check Zed logs for error messages
2. Verify WebAssembly build completed successfully
3. Ensure `extension.toml` is valid
4. Restart Zed after changes

### LSP Server Not Starting

1. Verify mdbook-lint is installed: `which mdbook-lint`
2. Test LSP server manually: `mdbook-lint lsp --help`
3. Check extension logs for startup errors

### No Diagnostics Showing

1. Ensure file is saved (LSP activates on save)
2. Check file extension is `.md` or `.markdown`
3. Verify mdbook-lint finds violations: `mdbook-lint lint your-file.md`

### Performance Issues

1. Check for very large markdown files
2. Consider rule configuration to disable unused rules
3. Monitor Zed resource usage