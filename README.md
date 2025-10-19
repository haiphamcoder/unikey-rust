# UniKey Rust

A modern, cross-platform Vietnamese Input Method written in Rust, based on the original UniKey project.

## 🎯 Project Overview

UniKey Rust is a complete rewrite of the popular UniKey Vietnamese input method in Rust, designed to be:
- **Cross-platform**: Linux, Windows, macOS support
- **Memory-safe**: Leveraging Rust's ownership system
- **High-performance**: Near C/C++ performance with better safety
- **Modern**: Clean architecture with excellent error handling
- **Extensible**: Modular design for easy customization

## 🏗️ Architecture

### Core Components

```
unikey-rust/
├── crates/
│   ├── unikey-core/           # Core engine and input processing
│   ├── unikey-encoding/       # Character set conversion
│   ├── unikey-macro/          # Macro system
│   ├── unikey-input-methods/  # Telex, VNI, VIQR implementations
│   ├── unikey-platform-common/# Common platform abstractions
│   ├── unikey-linux/          # Linux XIM/Wayland/GTK integration
│   ├── unikey-windows/        # Windows IME integration
│   └── unikey-macos/          # macOS Input Method integration
├── examples/                  # Example applications
├── tests/                     # Integration tests
├── docs/                      # Documentation
├── assets/                    # Data files (macros, keymaps, charsets)
└── scripts/                   # Build and deployment scripts
```

## 🚀 Roadmap

### Phase 1: Core Foundation (Weeks 1-4)
**Goal**: Establish core engine and basic input processing

#### Week 1: Project Setup & Core Types
- [x] Project scaffolding and workspace setup
- [ ] Core data structures (`VnLexiName`, `VowelSeq`, `KeyEvent`)
- [ ] Character set definitions and mappings
- [ ] Basic error handling and logging

#### Week 2: Input Method Framework
- [ ] Abstract input method trait
- [ ] Key event classification system
- [ ] Basic input processing pipeline
- [ ] Unit tests for core functionality

#### Week 3: Telex Implementation
- [ ] Telex input method implementation
- [ ] Vowel sequence processing
- [ ] Tone mark handling
- [ ] Special character processing (ă, â, đ, etc.)

#### Week 4: VNI & VIQR Implementation
- [ ] VNI input method implementation
- [ ] VIQR input method implementation
- [ ] Input method switching
- [ ] Comprehensive testing

### Phase 2: Encoding & Conversion (Weeks 5-6)
**Goal**: Complete character set conversion system

#### Week 5: Encoding Framework
- [ ] Character set conversion traits
- [ ] Unicode handling (UTF-8, UTF-16, UCS-2)
- [ ] Vietnamese character mappings
- [ ] Composite character support

#### Week 6: Legacy Encoding Support
- [ ] TCVN3, VPS, VISCII support
- [ ] VNI-Win, BKHCM support
- [ ] VIQR encoding
- [ ] Windows CP1258 support

### Phase 3: Advanced Features (Weeks 7-8)
**Goal**: Implement advanced features and optimizations

#### Week 7: Macro System
- [ ] Macro definition and storage
- [ ] Macro lookup and replacement
- [ ] File I/O for macro persistence
- [ ] Macro editor utilities

#### Week 8: Spell Checking & Smart Features
- [ ] Vietnamese word detection
- [ ] Non-Vietnamese sequence handling
- [ ] Free marking support
- [ ] Modern style processing

### Phase 4: Platform Integration (Weeks 9-12)
**Goal**: Platform-specific integrations

#### Week 9: Linux XIM Integration
- [ ] XIM server implementation
- [ ] X11 event handling
- [ ] System tray integration
- [ ] Configuration management

#### Week 10: Linux Wayland & GTK
- [ ] Wayland input method protocol
- [ ] GTK4 input method module
- [ ] Modern Linux desktop integration
- [ ] Flatpak/AppImage packaging

#### Week 11: Windows Integration
- [ ] Windows IME framework
- [ ] System tray and notifications
- [ ] Registry configuration
- [ ] Windows-specific optimizations

#### Week 12: macOS Integration
- [ ] macOS Input Method framework
- [ ] System preferences integration
- [ ] macOS-specific UI components
- [ ] App Store packaging

### Phase 5: Polish & Optimization (Weeks 13-16)
**Goal**: Performance optimization and user experience

#### Week 13: Performance Optimization
- [ ] Profiling and benchmarking
- [ ] Memory usage optimization
- [ ] Input latency reduction
- [ ] CPU usage optimization

#### Week 14: User Experience
- [ ] Configuration GUI
- [ ] Hotkey management
- [ ] Status indicators
- [ ] User documentation

#### Week 15: Testing & Quality Assurance
- [ ] Comprehensive test suite
- [ ] Fuzzing and stress testing
- [ ] Cross-platform testing
- [ ] Performance regression testing

#### Week 16: Release Preparation
- [ ] Documentation completion
- [ ] Packaging for all platforms
- [ ] Release notes and migration guide
- [ ] Community feedback integration

## 🛠️ Development Guidelines

### Code Organization
- **Core logic** in `unikey-core`
- **Platform-specific code** in respective platform crates
- **Shared utilities** in `unikey-platform-common`
- **Data files** in `assets/` directory

### Testing Strategy
- **Unit tests** for each module
- **Integration tests** for cross-module functionality
- **Property-based testing** for input processing
- **Platform-specific tests** for each OS

### Performance Targets
- **Input latency**: < 5ms for key processing
- **Memory usage**: < 10MB baseline
- **CPU usage**: < 1% idle, < 5% during typing
- **Startup time**: < 100ms

## 🔧 Building and Running

### Prerequisites
- Rust 1.70+ (stable)
- Platform-specific dependencies (see individual crate READMEs)

### Quick Start
```bash
# Clone the repository
git clone https://github.com/unikey-rust/unikey.git
cd unikey

# Build all crates
cargo build --release

# Run tests
cargo test

# Run examples
cargo run --example simple-gui
```

### Platform-specific Builds
```bash
# Linux (XIM)
cargo build --release -p unikey-linux

# Windows (IME)
cargo build --release -p unikey-windows

# macOS (Input Method)
cargo build --release -p unikey-macos
```

## 📚 Documentation

- [Architecture Overview](docs/architecture.md)
- [Input Methods Guide](docs/input-methods.md)
- [Platform Integration](docs/platform-integration.md)
- [API Reference](docs/api-reference.md)
- [Contributing Guide](docs/contributing.md)

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Areas for Contribution
- **Core engine improvements**
- **New input methods**
- **Platform integrations**
- **Performance optimizations**
- **Documentation**
- **Testing**

## 📄 License

This project is licensed under the GNU General Public License v2.0 or later - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Original UniKey project** by Pham Kim Long
- **Vietnamese input method community**
- **Rust ecosystem contributors**

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/unikey-rust/unikey/issues)
- **Discussions**: [GitHub Discussions](https://github.com/unikey-rust/unikey/discussions)
- **Documentation**: [Project Wiki](https://github.com/unikey-rust/unikey/wiki)

---

**Status**: 🚧 In Active Development

**Current Phase**: Phase 1 - Core Foundation

**Next Milestone**: Basic Telex input method implementation
