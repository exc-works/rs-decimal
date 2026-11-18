---
locale: en-GB
source_revision: 13b73a8f1a7c329ddb10d60fb148021dcab79998f09acc291519f00e02fa1abd
translation_provenance: en-US-reviewed-reuse
api_id: "NullDecimal::impl Type < Postgres >::compatible"
---
# NullDecimal::impl Type < Postgres >::compatible

Use the native SQLx PostgreSQL NUMERIC integration.

```rust
fn compatible (ty : & PgTypeInfo) -> bool
```

## Normal usage and caveats

Requires the postgres feature and SQLx 0.8 PostgreSQL types. This implementation is native-only: the browser playground does not open database connections. type_info() returns NUMERIC OID 1700. compatible(ty) accepts that exact OID, not float8, integer or arbitrary textual types; no implicit float conversion is promised.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-nulldecimal-impl-type-postgres-compatible" locale="en-GB" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
