# Documentation preview

This VitePress site is a preview, not a claim of complete method or translation
coverage. The API catalogue comes from the reviewed Rust inventory and explicitly
shows planned work. The release coverage gate remains `tools/check-web-api/check.py
--release` and is expected to fail until every planned family is delivered.

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
one shared executable Rust snippet; future method pages use shared examples and
are subject to the separate API release gate. Source hashes detect outdated
prose, not the linguistic quality of a translation; human editorial review is
still useful before the final release.
