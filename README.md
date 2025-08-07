# mdbook-lint Zed Extension

A Zed editor extension that provides real-time markdown linting for mdBook projects using the mdbook-lint LSP server.

## Features

- **Real-time markdown linting**: Get instant feedback on markdown issues as you type
- **mdBook-specific rules**: Enhanced linting for mdBook projects including SUMMARY.md validation and cross-reference checking
- **Standard markdown rules**: Comprehensive support for 59 standard markdown rules (MD001-MD059)
- **Automatic project detection**: Automatically detects mdBook projects and enables enhanced features
- **Configuration support**: Respects mdbook-lint configuration files in your project

## Installation

### Prerequisites

First, install the mdbook-lint command-line tool:

```bash
cargo install mdbook-lint
```

### Install Extension

1. Open Zed
2. Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux) to open the command palette
3. Type "Extensions" and select "zed: extensions"
4. Search for "mdbook-lint" and click "Install"

Alternatively, you can install from the command line:

```bash
# Clone and install the extension manually
git clone https://github.com/joshrotenberg/mdbook-lint-zed
cd mdbook-lint-zed
# Follow Zed extension development workflow
```

## Usage

Once installed, the extension will automatically:

1. **Detect markdown files**: Activate when you open `.md` or `.markdown` files
2. **Start LSP server**: Launch the mdbook-lint LSP server in the background
3. **Show diagnostics**: Display linting violations as you type with:
   - Red underlines for errors
   - Yellow underlines for warnings
   - Blue underlines for informational messages
4. **Provide hover information**: Hover over violations to see detailed explanations
5. **Enhanced mdBook support**: When working in an mdBook project (detected by `book.toml` or `SUMMARY.md`), additional rules become active

## Configuration

The extension respects mdbook-lint configuration files in your project:

- `mdbook-lint.toml` (recommended)
- `mdbook-lint.yaml`
- `mdbook-lint.json`

Example configuration:

```toml
# mdbook-lint.toml
fail_on_warnings = false
fail_on_errors = true

[core]
# Enable only specific rules
enabled_rules = ["MD001", "MD013", "MDBOOK001"]

# Or disable specific rules
disabled_rules = ["MD002"]

# Enable rule categories
enabled_categories = ["structure", "formatting"]

# Markdownlint compatibility mode
markdownlint_compatible = false
```

## Rules

The extension supports all mdbook-lint rules:

### Standard Rules (MD001-MD059)
- **MD001**: Heading levels should only increment by one level at a time
- **MD013**: Line length restrictions
- **MD040**: Fenced code blocks should have a language tag
- And 56 more...

### mdBook Rules (MDBOOK001-007)
- **MDBOOK001**: Fenced code blocks should have language tags
- **MDBOOK002**: Relative links should point to existing files
- **MDBOOK003**: SUMMARY.md structure validation
- **MDBOOK004**: Heading title uniqueness across documents
- **MDBOOK005**: Orphaned files detection
- **MDBOOK006**: Cross-reference link validation
- **MDBOOK007**: Include file validation

## mdBook Project Features

When working in an mdBook project (containing `book.toml` or `SUMMARY.md`), you get:

- **SUMMARY.md validation**: Ensures proper structure and syntax
- **Cross-reference checking**: Validates links between markdown files
- **Include file validation**: Checks `{{#include}}` statements
- **Orphaned file detection**: Identifies markdown files not referenced in SUMMARY.md
- **Anchor link validation**: Ensures internal links point to existing headings

## Performance

The extension is designed for performance:

- **Single-pass parsing**: Documents are parsed once and reused across all rules
- **Incremental updates**: Only re-analyzes changed content
- **Low memory footprint**: Efficient AST handling and minimal retention
- **Fast startup**: Quick LSP server initialization

## Troubleshooting

### LSP Server Not Starting

If the extension can't find mdbook-lint:

1. Ensure mdbook-lint is installed: `cargo install mdbook-lint`
2. Check it's in your PATH: `which mdbook-lint`
3. Restart Zed after installation

### No Diagnostics Showing

1. Check the Zed output panel for LSP server logs
2. Verify your markdown files have proper extensions (`.md`, `.markdown`)
3. Ensure the file is saved (LSP activates on file save)

### Performance Issues

1. Check for very large markdown files (>1MB)
2. Consider disabling rules you don't need via configuration
3. Use `enabled_rules` instead of `disabled_rules` for better performance

## Development

To contribute to this extension:

```bash
git clone https://github.com/joshrotenberg/mdbook-lint-zed
cd mdbook-lint-zed
cargo build
```

The extension is built using:
- Rust and WebAssembly
- Zed Extension API
- mdbook-lint LSP server

## Links

- [mdbook-lint Repository](https://github.com/joshrotenberg/mdbook-lint)
- [mdbook-lint Documentation](https://joshrotenberg.github.io/mdbook-lint/)
- [Zed Extension Development](https://zed.dev/docs/extensions/developing-extensions)

## License

This extension is released under the same license as mdbook-lint: MIT OR Apache-2.0.