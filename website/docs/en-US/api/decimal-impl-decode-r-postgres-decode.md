---
locale: en-US
source_revision: 21da10a6e76a84c5146b6a083d73a886edce7505fb47206e66d109e57f693aeb
api_id: "Decimal::impl Decode < 'r , Postgres >::decode"
---
# Decimal::impl Decode < 'r , Postgres >::decode

Use the native SQLx PostgreSQL NUMERIC integration.

```rust
fn decode (value : PgValueRef < 'r >) -> Result < Self , BoxDynError >
```

## Normal usage and caveats

Requires the postgres feature and SQLx 0.8 PostgreSQL types. This implementation is native-only: the browser playground does not open database connections. decode(PgValueRef) returns `Result<Self, BoxDynError>`. SQLx supplies the borrowed database value; do not fabricate its private wire representation. Binary and text NUMERIC preserve value and display scale, including scaled zero. SQL NULL is rejected; use NullDecimal or `Option<Decimal>` for nullable columns. Non-null results are initialized. Malformed or nonfinite NUMERIC values are rejected without rounding. This example runs against a dedicated PostgreSQL server in CI; no external database is required for the browser.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-decode-r-postgres-decode" locale="en-US" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. Run this example with a dedicated PostgreSQL server, DATABASE_URL and `cargo run --locked -p decimal-web-examples --features postgres-live`. Without that profile it is explicitly reported as skipped, not counted as a passed database check.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
