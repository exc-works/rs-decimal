# Documentation and browser playground

This static VitePress site documents all 355 API entries in eight languages.
The catalogue identifies browser-supported methods and native-only integrations;
Rust/Wasm performs playground decimal operations. `npm run check:release` checks
complete API coverage and current translations. See [DEPLOYMENT.md](DEPLOYMENT.md)
for the two-origin static package, version checks, headers and release verification.

Use Node 22 and `npm ci --ignore-scripts`, then `npm run dev` in this directory.
`npm run check:i18n`, `node --test tests/i18n.test.mjs` and `npm run build` validate
the shared pages. Set `DOCS_BASE=/rs-decimal/` to build for a subpath. The static
output is `docs/.vitepress/dist`; pass the same `DOCS_BASE` to `npm run preview`
when previewing that build. Local full-text search needs no backend.
Run `cargo run --locked --example web_getting_started` from the repository root
to execute the single shared Rust example included by all eight guides.

## Translation changes

The reference locale is en-US. `translation-manifest.json` lists shared pages;
each translation records the SHA-256 of the reference Markdown body (excluding
frontmatter) in `source_revision`. Update that field only after reviewing the
translation against the new reference meaning. A stale or missing page fails CI;
there is no silent English fallback masquerading as a translated page. en-GB was
reviewed from en-US with British spelling and retains the same provenance.

Interface strings are complete per locale in `locales.mjs`. Rust names, code,
input syntax and API identifiers remain unchanged between languages. Guides use
one shared executable Rust snippet; method pages use shared examples and
are subject to the separate API release gate. Source hashes detect outdated
prose, not the linguistic quality of a translation; human editorial review is
still useful before the final release.

## Build dependency maintenance

Vue is pinned to 3.5.45. VitePress 1.6.4 still requests Vite 5, so this site
pins a tested Vite 6.4.6 override to include the maintained security fixes; the
Vue plugin supports Vite 6. Build and browser checks cover this override.

As of 2026-11-18, `npm audit` still reports the transitive build-only
[picomatch extglob ReDoS advisory](https://github.com/advisories/GHSA-fc5q-39xc-vjpf)
for the latest 4.0.7. Upstream lists no patched version. This project supplies
reviewed repository glob literals, not visitor-controlled patterns, to Vite;
picomatch is absent from the deployed static client. No public dev server is
deployed. The remaining finding is recorded rather than claiming a clean audit;
update the lockfile when an upstream patch exists. Never route playground input
through the build tooling or accept untrusted glob patterns.

`tests/browser-site.py` starts a local preview and checks eight locale routes,
API filtering, language-preserving navigation, local search and narrow-screen
layout. Set `CHROMEDRIVER` and `TEST_CHROME_BINARY` to matching executables.

## Editor and local execution

The playground loads Monaco after client mounting and only on its own route.
JavaScript completion and parameter help use the bridge declarations; unsupported
methods are not advertised. The editor, TypeScript worker and locale bundles are
local assets. The initial development build has roughly 3.2 MiB of editor code
and a 6.7 MiB TypeScript worker before compression; ordinary documentation routes
do not load these chunks. Monaco 0.57.0 uses DOMPurify, overridden to 3.4.18 to
include its security fixes. The previously documented build-only picomatch
advisory remains; no clean audit claim is made.

Set `DOCS_RUNNER_URL` to the trusted separate-origin runner deployment when
building docs. Both packages must be built at the same Git commit. For a local
check, build Wasm, package the runner with parent `http://127.0.0.1:4210`, then
build docs with `DOCS_BASE=/rs-decimal/ DOCS_RUNNER_URL=http://127.0.0.1:4211/`.
`tests/browser-editor.py` serves these packages and tests the actual UI. Without
runner configuration, editing/import/export remain available and Run is visibly
disabled. An editor route never runs a restored draft or imported file by itself.

`show(value)`/`print(value)` snapshot values immediately; `assert(condition,
message?)` reports a script assertion error. Output is rendered as escaped text;
bounded partial output survives a later error. Ctrl/Cmd+Enter runs the source.
Stop terminates execution; Reset restores the initial example. Local storage
holds a validated versioned source draft, and session storage holds bounded
results. File import validates UTF-8 size before use; export preserves source.
Locale changes preserve draft/results. Monaco's built-in labels load in the
selected language; switching its language reloads the route after state saving
because some labels are initialized once. Browser storage can be unavailable;
the UI reports draft-save failure and still allows file export.
