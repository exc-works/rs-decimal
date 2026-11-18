# Deploy to GitHub Pages

The docs, Monaco editor and Rust/Wasm playground are one static site. No second
origin, custom response headers, server, login or database is required.

## Automatic publishing

In this repository's **Settings → Pages → Build and deployment**, select
**GitHub Actions** as the source. `.github/workflows/pages.yml` builds and deploys
on pushes to `main`, or through its manual workflow action. The build reads the
Pages base path, checks all 355 API entries and eight current translations, and
builds Wasm, runner and docs from one clean commit. The deployment job publishes
only `website/release-dist/docs/` with the standard GitHub Pages actions.

The project URL is `https://exc-works.github.io/rs-decimal/` unless the repository
has a custom domain. No domain purchase is necessary. The workflow needs Pages
publishing enabled and the `github-pages` environment must allow `main`.

## Local build

Install Node 22+, Python 3, Rust with `wasm32-unknown-unknown`, and
`wasm-bindgen-cli` exactly 0.2.105. From a clean committed checkout:

```sh
npm --prefix website ci --ignore-scripts
DOCS_BASE=/rs-decimal/ npm --prefix website run build:release
```

Use `DOCS_BASE=/` for an origin-root deployment. The package's `docs/` directory
contains all published files, including `runner/sandbox.json`. `release.json`
records the source commit, base path, Wasm measurements and each file's SHA-256.
Old `DOCS_ORIGIN`, `DOCS_RUNNER_URL` and `RUNNER_PARENT_ORIGINS` settings are rejected
by the release command; no hosting origins need to be configured.

Preserve `.html` routes, directory `index.html`, and assets. Do not rewrite missing
files to the homepage. GitHub Pages supplies ordinary static HTTP responses;
no `_headers` file or server-side configuration is used.

## Browser execution

The parent page fetches a versioned trusted bootstrap package from its own site.
It creates an iframe with only `sandbox="allow-scripts"`, giving it an opaque
origin. User source is sent through a private MessagePort and never interpolated
into HTML. The frame creates a fresh Blob Worker containing the bundled bridge
and Wasm for each Run. The worker inherits the frame's meta CSP, which blocks
network requests and external imports. Worker/SharedWorker constructors are
irreversibly disabled before user code to prevent creating descendants.

The worker cannot access the document, parent DOM or the site's origin storage.
Both frame and parent enforce execution deadlines; Stop terminates the active
worker. Rust resource limits and bounded text output remain enabled. Shared
examples never run automatically. These controls limit accidental and malicious
scripts, but do not constitute a hard quota for arbitrary JavaScript heap usage.

A runner/website commit mismatch disables execution rather than running stale
code. Publish the whole directory as one artifact; an older open tab may need a
reload after deployment. Rollback means redeploying the complete older package.

## Verification

After a local build, install and run the pinned browser engines:

```sh
cd website
npx playwright install --with-deps chromium firefox webkit
npm run test:browsers
```

Tests use a single localhost static origin without custom response headers. They
exercise real Rust examples, opaque-origin/storage separation, network/import
blocking, immutable child-worker restrictions, cancellation, recovery, eight
locale routes and the production editor. Removing the meta CSP in a negative
control must cause the runtime to refuse user code. `.cache/browser-engines.json`
records tested build and browser versions. The Chrome suites additionally cover
search, completion, parameter hints and invalid share/draft boundaries.

After publishing, open a nested method page and Playground, explicitly Run an
example, Stop a loop and check a shared link does not execute automatically.
Check `build.json` and `runner/manifest.json` identify the same commit.
