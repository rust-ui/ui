# Prerequisites

## Rust (Nightly)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Switch to nightly
rustup default nightly

# Add WASM target
rustup target add wasm32-unknown-unknown
```

## Dioxus CLI

```bash
cargo install dioxus-cli
```

## Just

```bash
cargo install just
```

## PostgreSQL

**macOS (Homebrew)**
```bash
brew install postgresql@17
brew services start postgresql@17
```

**Windows**

Download and install from [postgresql.org/download/windows](https://www.postgresql.org/download/windows/).

**Linux (Debian/Ubuntu)**
```bash
sudo apt install postgresql postgresql-contrib
sudo systemctl start postgresql
```

The database itself is created via `just reset_db` (see README).

## pnpm

```bash
npm install -g pnpm
```
