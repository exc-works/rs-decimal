# Static documentation and playground deployment

The output is two static sites. Decimal execution happens in a fresh browser
Worker running Rust/Wasm. No application server, database or user-code execution
service is needed. Use a dedicated runner origin without account cookies,
authenticated endpoints or unrelated content; preferably use a separate site.

## Build a package

Use a clean committed checkout, Node 22+, Python 3, Rust with the
`wasm32-unknown-unknown` target, and `wasm-bindgen-cli` exactly 0.2.105.
From the repository root:

```sh
npm --prefix website ci --ignore-scripts
DOCS_ORIGIN=https://docs.example.org \
DOCS_BASE=/rs-decimal/ \
DOCS_RUNNER_URL=https://runner.example.net/ \
npm --prefix website run build:release
```

Replace the example origins with the actual hosting origins. The runner is served
at its origin root. The docs base is `/` or a slash-terminated path. The script
requires all 355 API entries and all eight translations, builds Wasm, runner and
docs from one commit, validates their configuration and writes
`website/release-dist/`. `release.json` contains the commit, origins, base path,
Wasm measurements and SHA-256/size of every packaged file except itself.
Store that manifest with the CI run and its review/check evidence.

The package is a build artifact, not an automatic publishing operation or proof
that a hosting provider applies its headers. GitHub CI validates the localhost
configuration; run the command again with production origins before publishing.
`RELEASE_ALLOW_LOCALHOST=1` permits explicit localhost proof builds, marked
`local: true` in the manifest. Never publish that package as a production build.

## Hosting contract

- Upload `docs/` under `DOCS_BASE` on the docs origin. Preserve `.html` routes,
  directory `index.html`, all assets and local search indexes. Do not rewrite
  missing files to the home page. Serve JS as JavaScript and Wasm as
  `application/wasm`; enable gzip/Brotli at the static host if available.
- Upload `runner/` at the runner origin root. Apply `runner/headers.json` as
  response headers, or use `_headers` on a host that supports that format.
  Merely uploading either file does not configure other servers.
- Apply `/*` headers to all runner responses and the exact path-specific headers
  to `/`, `/index.html` and `/worker.js`. Do not apply the iframe's CSP to the
  worker: CSP policies combine restrictively, and the worker intentionally needs
  `unsafe-eval` while denying network, imports and nested workers. Preserve
  `frame-ancestors` with the explicit docs origin. Do not substitute meta CSP.
- The runner uses `Cache-Control: no-store`. Serve docs HTML and `build.json`
  with revalidation; fingerprinted docs assets may use immutable caching.
  Do not add a service worker that can retain a different release.
- Replace each site's files atomically where supported. Deploy the matching
  runner and docs packages together. During a cross-site rollout, a commit
  mismatch intentionally prevents execution. Roll back both sites to a matching
  package if verification fails; do not disable the version handshake.

GitHub Pages can host docs. A runner host must support the required per-path
HTTP response headers. No hosting account or deployment is created by this repo.

## Verify before publishing

Build a localhost package with docs origin `http://127.0.0.1:4210`, base
`/rs-decimal/`, runner URL `http://127.0.0.1:4211/`, and
`RELEASE_ALLOW_LOCALHOST=1`. Install the pinned Playwright browsers and run:

```sh
cd website
npx playwright install --with-deps chromium firefox webkit
npm run test:browsers
```

The test serves the generated files with their actual headers, executes the
shared Rust examples in all three engines, verifies cancellation/error recovery,
network/import isolation and the missing-header control, then exercises all
locale method routes and the production editor. It writes
`.cache/browser-engines.json` with engine versions and passed counts. The existing
Chrome suites additionally cover search, completion, parameter hints and invalid
share/draft boundaries. [Playwright browser installation](https://playwright.dev/docs/browsers)
documents matching binaries and Linux system dependencies.

The native checks are `cargo run --locked -p decimal-web-examples` and, with a
dedicated test PostgreSQL database, `cargo run --locked -p decimal-web-examples
--features postgres-live`. The default run explicitly skips four live-database
examples; it is not a substitute for the PostgreSQL CI job.

After uploading, inspect response headers for `/`, `/index.html` and `/worker.js`
on the actual runner origin. Open nested docs routes in every locale, run the
default example, Stop a long loop, and open a shared example (which must await an
explicit Run). Check that docs `build.json`, runner `manifest.json` and packaged
Wasm metadata identify the same commit. Verify a worker `fetch` attempt is blocked
and creates no request at the runner host. A local proof cannot detect hosting
rewrites or stripped headers, so these are post-upload acceptance checks.
