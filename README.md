# chessbik-server
[Releases](https://github.com/necromfox/chessbik-server/releases)

# requirements to build
- [rustup](https://www.rust-lang.org/tools/install)
- wasm32-unknown-unknown (run "rustup target add wasm32-unknown-unknown")
- [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/reference/cli.html)
- [wasm-opt](https://github.com/WebAssembly/binaryen)
- gzip

# IMPORTANT!
When building, always explicitly specify target, since "cargo build --release" would fail to copy static assets.
