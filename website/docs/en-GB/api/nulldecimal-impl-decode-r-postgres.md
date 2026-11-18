---
locale: en-GB
source_revision: 54175f7d24af1ed024b6094baba67179a4412d7b895f4da6835c5302cb6117c7
translation_provenance: en-US-reviewed-reuse
api_id: "NullDecimal::impl Decode < 'r , Postgres >"
---
# NullDecimal::impl Decode < 'r , Postgres >

Use the native SQLx PostgreSQL NUMERIC integration.

```rust
impl< 'r > Decode < 'r , Postgres > for NullDecimal
```

## Normal usage and caveats

Requires the postgres feature and SQLx 0.8 PostgreSQL types. This implementation is native-only: the browser playground does not open database connections. decode(PgValueRef) returns `Result<Self, BoxDynError>`. SQLx supplies the borrowed database value; do not fabricate its private wire representation. Binary and text NUMERIC preserve value and display scale, including scaled zero. SQL NULL returns invalid NullDecimal with uninitialized inner zero; non-null values become valid. Malformed or nonfinite NUMERIC values are rejected without rounding. This example runs against a dedicated PostgreSQL server in CI; no external database is required for the browser.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-nulldecimal-impl-decode-r-postgres" locale="en-GB" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. Run this example with a dedicated PostgreSQL server, DATABASE_URL and `cargo run --locked -p decimal-web-examples --features postgres-live`. Without that profile it is explicitly reported as skipped, not counted as a passed database check.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
