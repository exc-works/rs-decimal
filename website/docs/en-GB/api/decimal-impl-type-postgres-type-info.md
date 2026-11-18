---
locale: en-GB
source_revision: 841ef20b63498074bad543fcbeb5b16a645ffde5b38df63feb8a348e6536be41
translation_provenance: en-US-reviewed-reuse
api_id: "Decimal::impl Type < Postgres >::type_info"
---
# Decimal::impl Type < Postgres >::type_info

Use the native SQLx PostgreSQL NUMERIC integration.

```rust
fn type_info () -> PgTypeInfo
```

## Normal usage and caveats

Requires the postgres feature and SQLx 0.8 PostgreSQL types. This implementation is native-only: the browser playground does not open database connections. type_info() returns NUMERIC OID 1700. compatible(ty) accepts that exact OID, not float8, integer or arbitrary textual types; no implicit float conversion is promised.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-type-postgres-type-info" locale="en-GB" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
