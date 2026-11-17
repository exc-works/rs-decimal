# Website API contracts

`rust-api.json` is generated independently from Rust source by the pinned `syn`
parser in `tools/check-web-api`. It includes conditional APIs without enabling
their runtime dependencies, excludes private/test-only types, records explicit
trait methods and derives, and expands this repository's integer conversion
macro. Unknown item macros, public aliases/traits/statics and glob exports fail
closed until the extractor supports their visibility. Re-exports are recorded
without recursively claiming third-party APIs. This is an inventory, not a Rust
compiler or a general macro expander.

`coverage.json` is the reviewed plan and implementation evidence, keyed by the
same IDs. A planned entry is not a working bridge. An implemented entry requires
a bridge signature (unless Rust-only), example/test references, and en-US/zh-Hans
pages. `--release` additionally rejects all planned entries and requires all eight
locale pages. Translation freshness and execution of examples are separate gates
added with WEB-05 and the relevant bridge PRs; file existence alone is not proof
that an example passed.

```sh
cargo run --locked -p decimal-web-api -- --write
cargo run --locked -p decimal-web-api -- --check
python3 tools/check-web-api/check.py
python3 tools/check-web-api/test_check.py
```

Review the source inventory diff before changing dispositions. Each source ID
has a unique `doc_id` used as `website/docs/<locale>/api/<doc_id>.md`, an owning
WEB PR, a `direct`/`adapter`/`rust-only` disposition, and a mapping explanation
for the latter two. Trait-derived defaults/cloning can gain named bridge adapters
when implemented; Rust trait syntax itself remains a documented difference.

## Method page contract

Each method page has frontmatter `api_id`, `locale`, and `source_revision` (the
reference explanation's revision/hash), followed by purpose, normal Rust usage
and output, runnable bridge example, parameters/return value, everyday pitfalls,
errors/special boundaries and related APIs. Code and exact expected results are
referenced from examples, not independently translated. See the
[design](../../docs/web-playground-design.md) for terminology and state rules.

## Example envelope, schema version 1

Example files live at `website/examples/<id>.json` and contain exactly:

```json
{
  "schema_version": 1,
  "id": "exact-product",
  "api_ids": ["Decimal::new", "Decimal::new_from_string", "Decimal::mul_exact"],
  "rust": "let value = Decimal::new_from_string(\"12.350\")?.mul_exact(&Decimal::new(3))?;",
  "javascript": "const value = Decimal.new_from_string(\"12.350\").mul_exact(Decimal.new(3n));",
  "expected": {
    "coefficient": "37050",
    "scale": "3",
    "initialized": true
  }
}
```

The Rust harness supplies imports and a Result-returning context. The browser
harness supplies the bridge. Both capture `value`; error/state-transition
examples will add explicit typed expected outcomes without erasing distinctions
between returned errors and traps. Store arbitrary integers as strings and
special floating values as tagged bit representations. This illustrative envelope
is not implementation evidence until both runners execute and compare it.
