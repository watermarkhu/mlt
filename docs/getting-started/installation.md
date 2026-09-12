---
icon: lucide/download
---

# Installation

## From source (Cargo)

mlt is written in Rust and currently installed from source via Cargo.

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (1.70+ recommended)
- A C compiler (required by tree-sitter build)

### Install

```bash
# Clone the repository
git clone https://github.com/watermarkhu/mlt.git
cd mlt

# Install the binary
cargo install --path crates/mlt_cli
```

This places the `mlt` binary in your Cargo bin directory (typically `~/.cargo/bin/`).

### Build without installing

```bash
# Build in release mode
cargo build --release

# Binary is at target/release/mlt
./target/release/mlt --version
```

## From crates.io (planned)

Once published:

```bash
cargo install mlt
```

## Verify Installation

```bash
mlt --version
```

## Next Steps

- [Quick Start](quickstart.md) — Get up and running with mlt
- [CLI Reference](../usage/cli.md) — Full command reference
- [Configuration](../configuration.md) — Customize mlt for your project
