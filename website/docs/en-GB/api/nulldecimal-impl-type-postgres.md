---
locale: en-GB
source_revision: 0830b6acb470b67cab11063b94daf8eafba63a697871936bf2d04b61331f067d
translation_provenance: en-US-reviewed-reuse
api_id: "NullDecimal::impl Type < Postgres >"
---
# NullDecimal::impl Type < Postgres >

Use the native SQLx PostgreSQL NUMERIC integration.

```rust
impl Type < Postgres > for NullDecimal
```

## Normal usage and caveats

Requires the postgres feature and SQLx 0.8 PostgreSQL types. This implementation is native-only: the browser playground does not open database connections. type_info() returns NUMERIC OID 1700. compatible(ty) accepts that exact OID, not float8, integer or arbitrary textual types; no implicit float conversion is promised.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-nulldecimal-impl-type-postgres" locale="en-GB" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
