# Contributing to mdbook-lint Zed Extension

Thank you for your interest in contributing to the mdbook-lint Zed extension! This document provides guidelines and information for contributors.

## Getting Started

1. **Prerequisites**:
   - [Rust](https://rustup.rs/) installed via rustup
   - WebAssembly target: `rustup target add wasm32-wasip1`
   - [Zed editor](https://zed.dev/) for testing
   - [mdbook-lint](https://github.com/joshrotenberg/mdbook-lint): `cargo install mdbook-lint`

2. **Fork and Clone**:
   ```bash
   git clone https://github.com/your-username/mdbook-lint-zed
   cd mdbook-lint-zed
   ```

3. **Build**:
   ```bash
   cargo build --target wasm32-wasip1
   ```

## Development Workflow

### Making Changes

1. **Create a branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following the code style guidelines below

3. **Test your changes**:
   ```bash
   # Check formatting
   cargo fmt --all -- --check
   
   # Run linting
   cargo clippy --all-targets --all-features -- -D warnings
   
   # Build WebAssembly
   cargo build --target wasm32-wasip1
   ```

4. **Test with Zed** (see DEVELOPMENT.md for detailed instructions)

### Code Style

- **Rust formatting**: Use `cargo fmt` 
- **Linting**: Ensure `cargo clippy` passes without warnings
- **Comments**: Add doc comments for public functions and complex logic
- **Error handling**: Use proper Result types and meaningful error messages

### Commit Messages

Use conventional commit format:
- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation changes
- `refactor:` for code refactoring
- `test:` for adding tests
- `ci:` for CI/CD changes

Example: `feat: add hover information for rule violations`

## Testing

### Automated Testing

The CI pipeline runs on every PR and includes:
- Code formatting check (`cargo fmt`)
- Linting (`cargo clippy`)
- WebAssembly compilation
- Cross-platform builds (Ubuntu, macOS, Windows)

### Manual Testing

1. **Build the extension**:
   ```bash
   cargo build --target wasm32-wasip1 --release
   ```

2. **Install locally** (see DEVELOPMENT.md for detailed steps):
   ```bash
   mkdir -p ~/.config/zed/extensions/mdbook-lint
   cp target/wasm32-wasip1/release/mdbook_lint_zed.wasm ~/.config/zed/extensions/mdbook-lint/
   cp extension.toml ~/.config/zed/extensions/mdbook-lint/
   ```

3. **Test with various scenarios**:
   - Regular markdown files
   - mdBook projects with violations
   - Configuration file discovery
   - LSP server startup and diagnostics

## Areas for Contribution

### High Priority
- **Performance optimizations**: Improve extension startup time
- **Configuration**: Enhanced configuration file support
- **Error handling**: Better error messages and recovery
- **Documentation**: Usage examples, screenshots, troubleshooting

### Medium Priority
- **Features**: Additional LSP capabilities (code actions, fixes)
- **UI/UX**: Better diagnostic presentation
- **Testing**: Automated integration tests
- **Compatibility**: Support for older Zed versions

### Low Priority
- **Customization**: Theming and appearance options
- **Advanced features**: Rule customization UI
- **Metrics**: Usage analytics and performance monitoring

## Pull Request Process

1. **Ensure CI passes**: All checks must be green
2. **Update documentation**: Update README.md, CHANGELOG.md if needed
3. **Test thoroughly**: Verify the extension works in Zed
4. **Write descriptive PR description**:
   - What changes were made
   - Why they were made
   - How to test the changes
   - Any breaking changes

### PR Template

```markdown
## Summary
Brief description of changes

## Changes Made
- List of specific changes
- Include any breaking changes

## Testing
- [ ] Builds successfully (`cargo build --target wasm32-wasip1`)
- [ ] Passes formatting check (`cargo fmt --check`)
- [ ] Passes linting (`cargo clippy`)
- [ ] Tested manually in Zed
- [ ] Works with mdBook projects
- [ ] Works with regular markdown files

## Screenshots/Videos
(If applicable, especially for UI changes)

## Related Issues
Closes #123
```

## Release Process

Releases are automated when version tags are pushed:

1. **Update version** in `Cargo.toml` and `extension.toml`
2. **Update CHANGELOG.md** with new version notes
3. **Create and push tag**:
   ```bash
   git tag v0.2.0
   git push origin v0.2.0
   ```
4. **GitHub Actions** will automatically create a release with WebAssembly artifacts

## Code of Conduct

### Our Standards

- **Be respectful**: Treat all contributors with respect
- **Be inclusive**: Welcome contributions from everyone
- **Be constructive**: Provide helpful feedback and suggestions
- **Be patient**: Remember that everyone is learning

### Unacceptable Behavior

- Harassment, discrimination, or personal attacks
- Spam, trolling, or off-topic discussions
- Publishing private information without permission
- Any behavior that would be inappropriate in a professional setting

## Getting Help

- **Issues**: Open a GitHub issue for bugs or feature requests
- **Discussions**: Use GitHub Discussions for questions and ideas
- **Documentation**: Check README.md and DEVELOPMENT.md first
- **Zed Community**: Join the [Zed Discord](https://discord.gg/zed) for general Zed questions

## Recognition

Contributors will be:
- Listed in the project's contributors
- Mentioned in release notes for significant contributions
- Given credit in any presentations or blog posts about the project

Thank you for contributing to mdbook-lint Zed extension! 🎉