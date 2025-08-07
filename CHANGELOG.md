# Changelog

All notable changes to the mdbook-lint Zed extension will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of mdbook-lint Zed extension
- Integration with mdbook-lint LSP server
- Real-time markdown linting for Zed editor
- Support for all 59 standard markdown rules (MD001-MD059)
- Support for 7 mdBook-specific rules (MDBOOK001-007)
- Automatic mdBook project detection
- Configuration file support (mdbook-lint.toml, .yaml, .json)
- Cross-platform support (macOS, Linux, Windows)

### Features
- Real-time diagnostics with error, warning, and info severity levels
- Hover information for detailed rule explanations
- Automatic LSP server discovery and startup
- Enhanced features for mdBook projects:
  - SUMMARY.md validation
  - Cross-reference checking
  - Include file validation
  - Orphaned file detection
  - Anchor link validation

## [0.1.0] - 2025-01-XX

### Added
- Initial implementation