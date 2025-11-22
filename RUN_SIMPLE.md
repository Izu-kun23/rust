# 🚀 Quick Start: Running NEXOS

## Prerequisites Installed ✅

- Rust (nightly)
- QEMU  
- bootimage
- rust-src component
- llvm-tools-preview

## Run Your OS

### Option 1: Using cargo run (Easiest)

```bash
source "$HOME/.cargo/env"
cd /Users/admin/Desktop/rust
cargo run
```

This will automatically:
1. Build your kernel
2. Create a bootable image
3. Launch QEMU with your OS

### Option 2: Using cargo bootimage

```bash
source "$HOME/.cargo/env"
cd /Users/admin/Desktop/rust
cargo bootimage
```

Then run QEMU manually:

```bash
qemu-system-x86_64 \
  -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-nexos.bin \
  -serial stdio
```

## What You'll See

A QEMU window will open showing:
- Blue boot screen
- "NEXOS v0.1.0" title
- Kernel initialization messages
- Your OS running!

## Exit QEMU

Press `Ctrl+C` in the terminal or close the QEMU window.

## Troubleshooting

### If `cargo run` doesn't work:

Try:
```bash
source "$HOME/.cargo/env"
cargo bootimage --runner qemu-system-x86_64
```

### If you get errors about nightly:

```bash
rustup override set nightly
```

