# chessbik-server
[Releases](https://github.com/necromfox/chessbik-server/releases)

# running
./chessbik-server (env CHESSBIK_SERVER_PORT defaults to 3000)

# requirements to build
- [rustup](https://www.rust-lang.org/tools/install)
- wasm32-unknown-unknown (run "rustup target add wasm32-unknown-unknown")
- [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/reference/cli.html)
- [wasm-opt](https://github.com/WebAssembly/binaryen)
- gzip

# IMPORTANT!
When building, make sure to edit WS_URL file.
When building, always explicitly specify target, since "cargo build --release" would fail to copy static assets.
