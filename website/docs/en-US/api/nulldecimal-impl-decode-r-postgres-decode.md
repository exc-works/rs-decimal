---
locale: en-US
source_revision: cf174440712c9de779826b8686589ffc2640afcea9cea0831e57bc66ade99c73
api_id: "NullDecimal::impl Decode < 'r , Postgres >::decode"
---
# NullDecimal::impl Decode < 'r , Postgres >::decode

Use the native SQLx PostgreSQL NUMERIC integration.

```rust
fn decode (value : PgValueRef < 'r >) -> Result < Self , BoxDynError >
```

## Normal usage and caveats

Requires the postgres feature and SQLx 0.8 PostgreSQL types. This implementation is native-only: the browser playground does not open database connections. decode(PgValueRef) returns `Result<Self, BoxDynError>`. SQLx supplies the borrowed database value; do not fabricate its private wire representation. Binary and text NUMERIC preserve value and display scale, including scaled zero. SQL NULL returns invalid NullDecimal with uninitialized inner zero; non-null values become valid. Malformed or nonfinite NUMERIC values are rejected without rounding. This example runs against a dedicated PostgreSQL server in CI; no external database is required for the browser.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-nulldecimal-impl-decode-r-postgres-decode" locale="en-US" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. Run this example with a dedicated PostgreSQL server, DATABASE_URL and `cargo run --locked -p decimal-web-examples --features postgres-live`. Without that profile it is explicitly reported as skipped, not counted as a passed database check.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
