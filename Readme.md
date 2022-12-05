<h1 style="text-align: center;">linktree</h1>

A json configurable linktree

## Usage

Dependency:
Install rust recommand way are [rustup](https://rustup.rs/)
```bash
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli
```

Get source:
```bash
git clone https://None/natir/linktree.git
cd linktree
```

Test:
```bash
trunk serve
```

Publish:
```bash
trunk build --release
```

This builds the app in release mode similar to `cargo build --release`.
You can also pass the `--release` flag to `trunk serve` if you need to get every last drop of performance.

The output will be located in the `dist` directory.

## Minimum supported Rust version

Currently the minimum supported Rust version is 1.65.
