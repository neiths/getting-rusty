# 🚀 Day 1 - Learning Rust

## 🛠️ Environment Setup

```bash
# Install Rust toolchain (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Update Rust to the latest version
rustup update

cargo new hello_rust
cd hello_rust

# Run the project

cargo run

# Build the project (debug)

cargo build

# Check for errors without building

cargo check

# Run tests

cargo test

# Build optimized release version

cargo build --release

# Generate documentation

cargo doc --open

curl -o .gitignore https://raw.githubusercontent.com/github/gitignore/main/Rust.gitignore
```
