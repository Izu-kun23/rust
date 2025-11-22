# 🚀 NEXOS - Next Generation Operating System

A modern operating system written in Rust, designed to rival Apple's macOS with:
- **Memory Safety**: Zero memory bugs through Rust's ownership system
- **Performance**: Near-C performance without the vulnerabilities
- **Modern Architecture**: Built for the future of computing

## 🎯 Goals

- Memory-safe kernel (no use-after-free, buffer overflows, etc.)
- Modern driver architecture
- Secure system daemons
- Zero-crash design philosophy

## 🏗️ Architecture

### Current Status (v0.1.0)
-  Bare-metal Rust kernel
-  VGA text mode display
-  Basic boot sequence

### Roadmap
- [ ] Memory management (paging, heap allocation)
- [ ] Interrupt handling
- [ ] Process scheduler
- [ ] Filesystem support
- [ ] Network stack
- [ ] GUI framework (rivaling macOS Aqua)

## 🔧 Building

First, install required tools:

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install bootimage for bootstrapping
cargo install bootimage

# Install rust-src component
rustup component add rust-src

# Install llvm-tools
rustup component add llvm-tools-preview
```

Then build:

```bash
cargo build
cargo bootimage
```

## 🚀 Running

```bash
cargo bootimage --runner qemu-system-x86_64
```

Or use QEMU directly:
```bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-nexos/debug/bootimage-nexos.bin
```

## 📚 Inspiration

- **Redox OS**: Full Rust-based OS
- **Linux**: Rust modules now supported
- **Microsoft**: Rust-based Windows components
- **Apple macOS**: Design philosophy and polish

## 🔐 Security First

This OS is built with security as a core principle:
- No unsafe code unless absolutely necessary
- Comprehensive bounds checking
- Memory-safe by default
- Modern cryptographic primitives


