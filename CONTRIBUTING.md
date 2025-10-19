# Contributing to UniKey Rust

Thank you for your interest in contributing to UniKey Rust! This document provides guidelines and information for contributors.

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+ (stable)
- Git
- Platform-specific dependencies (see individual crate READMEs)

### Setting Up Development Environment

1. **Fork and clone the repository**
   ```bash
   git clone https://github.com/your-username/unikey-rust.git
   cd unikey-rust
   ```

2. **Install dependencies**
   ```bash
   cargo build
   ```

3. **Run tests**
   ```bash
   cargo test
   ```

## 📋 Development Workflow

### 1. Choose an Issue
- Look for issues labeled `good first issue` for beginners
- Check the roadmap in README.md for current priorities
- Comment on the issue to express interest

### 2. Create a Branch
```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-number
```

### 3. Make Changes
- Follow the coding standards (see below)
- Write tests for new functionality
- Update documentation as needed

### 4. Test Your Changes
```bash
# Run all tests
cargo test

# Run specific tests
cargo test -p unikey-core

# Run with examples
cargo run --example your-example
```

### 5. Submit a Pull Request
- Push your branch to your fork
- Create a pull request with a clear description
- Link to related issues
- Request review from maintainers

## 📝 Coding Standards

### Rust Style
- Follow standard Rust formatting: `cargo fmt`
- Use clippy for linting: `cargo clippy`
- Write comprehensive documentation
- Use meaningful variable and function names

### Code Organization
- Keep modules focused and cohesive
- Use `pub use` for clean public APIs
- Implement traits for extensibility
- Prefer composition over inheritance

### Error Handling
- Use `thiserror` for custom error types
- Provide helpful error messages
- Use `anyhow` for application-level errors
- Log errors appropriately

### Testing
- Write unit tests for all public functions
- Use property-based testing where appropriate
- Test edge cases and error conditions
- Maintain good test coverage

## 🏗️ Architecture Guidelines

### Module Structure
```
crate/
├── src/
│   ├── lib.rs          # Public API and re-exports
│   ├── types.rs        # Core data structures
│   ├── traits.rs       # Abstract interfaces
│   ├── impl/           # Implementation modules
│   ├── error.rs        # Error types
│   └── tests/          # Integration tests
├── examples/           # Example applications
└── benches/           # Benchmark tests
```

### Dependencies
- Minimize external dependencies
- Use workspace dependencies when possible
- Prefer pure Rust implementations
- Document why each dependency is needed

## 🧪 Testing Guidelines

### Unit Tests
- Test individual functions and methods
- Use descriptive test names
- Test both success and failure cases
- Mock external dependencies

### Integration Tests
- Test module interactions
- Test end-to-end workflows
- Test platform-specific functionality
- Use real data when possible

### Property-Based Testing
- Use `proptest` for input validation
- Test invariants and properties
- Generate random test data
- Focus on edge cases

### Performance Testing
- Use `criterion` for benchmarks
- Measure key performance metrics
- Test with realistic data sizes
- Document performance characteristics

## 📚 Documentation

### Code Documentation
- Document all public APIs
- Use rustdoc format
- Include usage examples
- Explain complex algorithms

### README Files
- Each crate should have a README
- Include quick start examples
- Document platform-specific requirements
- Link to detailed documentation

### Architecture Documentation
- Document design decisions
- Explain module relationships
- Include diagrams where helpful
- Keep documentation up to date

## 🐛 Bug Reports

### Before Reporting
- Check existing issues
- Try the latest version
- Reproduce the issue
- Gather relevant information

### Bug Report Template
```markdown
**Describe the bug**
A clear description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior.

**Expected behavior**
What you expected to happen.

**Environment**
- OS: [e.g. Ubuntu 22.04]
- Rust version: [e.g. 1.70.0]
- UniKey version: [e.g. 0.1.0]

**Additional context**
Any other relevant information.
```

## ✨ Feature Requests

### Before Requesting
- Check existing issues and roadmap
- Consider if it fits the project scope
- Think about implementation complexity
- Consider alternative approaches

### Feature Request Template
```markdown
**Is your feature request related to a problem?**
A clear description of what the problem is.

**Describe the solution you'd like**
A clear description of what you want to happen.

**Describe alternatives you've considered**
A clear description of any alternative solutions.

**Additional context**
Any other context about the feature request.
```

## 🏷️ Issue Labels

- `bug`: Something isn't working
- `enhancement`: New feature or request
- `documentation`: Improvements to documentation
- `good first issue`: Good for newcomers
- `help wanted`: Extra attention is needed
- `priority:high`: High priority
- `priority:medium`: Medium priority
- `priority:low`: Low priority

## 📞 Getting Help

- **GitHub Discussions**: For questions and general discussion
- **GitHub Issues**: For bug reports and feature requests
- **Discord**: For real-time chat (if available)
- **Email**: For private matters

## 🎯 Areas for Contribution

### High Priority
- Core engine implementation
- Input method implementations
- Platform integrations
- Performance optimizations

### Medium Priority
- Documentation improvements
- Test coverage
- Example applications
- Developer tools

### Low Priority
- UI/UX improvements
- Additional input methods
- Platform-specific features
- Community tools

## 📋 Pull Request Checklist

- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] No breaking changes (or clearly documented)
- [ ] Performance impact considered
- [ ] Security implications considered

## 🏆 Recognition

Contributors will be recognized in:
- CONTRIBUTORS.md file
- Release notes
- Project documentation
- GitHub contributors page

Thank you for contributing to UniKey Rust! 🎉
