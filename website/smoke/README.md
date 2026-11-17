# Minimal Wasm proof (WEB-02)

Install the target and the version-matched CLI:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.105 --locked --root .cache/wasm-tools
python3 tools/build-web-wasm.py
python3 -m http.server 8000 --directory website
```

Open `http://localhost:8000/smoke/`. A passed report means a real browser executed
Rust parsing, exact multiplication, HalfEven rounding and an invalid-input error.
Initialization and execution measurements are shown separately. Artifact byte,
gzip and checksum measurements are in generated `public/wasm/build.json`.
Keep generated binaries out of Git. The build script records the checkout HEAD;
release builds must additionally require a clean checkout.

Run Rust-side browser tests with the matching wasm-bindgen-test-runner and a
ChromeDriver on PATH:

```sh
CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=.cache/wasm-tools/bin/wasm-bindgen-test-runner cargo test --locked -p decimal-playground-bridge --target wasm32-unknown-unknown
```

The core still uses no default features, and remains MSRV 1.85. The wrapper is a
feasibility slice: snapshot() is temporary inspection support, structured error
fields and complete types belong to WEB-03, untrusted execution/resource bounds
belong to WEB-04, and complete public API coverage remains planned. Do not expose
this smoke page as a user-code runner.
