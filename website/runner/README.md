# Browser runner

The runner is part of the same static GitHub Pages site as the documentation.
See [deployment instructions](../DEPLOYMENT.md) for the build and publishing flow.

`scripts/build-runner.mjs` bundles the bridge and Wasm into trusted bootstrap
code in `public/runner/sandbox.json`. The parent creates an opaque iframe with
only `sandbox="allow-scripts"`; the frame creates fresh Blob Workers that inherit
its meta CSP. No separate origin or HTTP response-header configuration is used.

User code is transferred through private ports and never enters the iframe HTML.
Network/import restrictions, locked child-worker constructors, bounded output,
version checks and parent-owned cancellation/deadlines are tested in Chromium,
Firefox and WebKit, including a negative control with the CSP removed.
