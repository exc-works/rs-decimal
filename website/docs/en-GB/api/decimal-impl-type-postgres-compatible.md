---
locale: en-GB
source_revision: 08d21be3db40ef9491657e8f165b85a45fc054adfc77bf6c387cc33e2075f02e
translation_provenance: en-US-reviewed-reuse
api_id: "Decimal::impl Type < Postgres >::compatible"
---
# Decimal::impl Type < Postgres >::compatible

Use the native SQLx PostgreSQL NUMERIC integration.

```rust
fn compatible (ty : & PgTypeInfo) -> bool
```

## Normal usage and caveats

Requires the postgres feature and SQLx 0.8 PostgreSQL types. This implementation is native-only: the browser playground does not open database connections. type_info() returns NUMERIC OID 1700. compatible(ty) accepts that exact OID, not float8, integer or arbitrary textual types; no implicit float conversion is promised.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-type-postgres-compatible" locale="en-GB" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
