---
locale: en-US
source_revision: 02d9f7c1451db386b5c6959aa8f2c1d403c00069d00df185eafd17eb7f7b188e
api_id: "Decimal::impl PgHasArrayType"
---
# Decimal::impl PgHasArrayType

Use the native SQLx PostgreSQL NUMERIC integration.

```rust
impl PgHasArrayType for Decimal 
```

## Normal usage and caveats

Requires the postgres feature and SQLx 0.8 PostgreSQL types. This implementation is native-only: the browser playground does not open database connections. array_type_info() returns NUMERIC[] OID 1231, enabling SQLx array bindings. NullDecimal array elements retain SQL NULL via validity; use native SQLx queries to encode/decode complete arrays.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-pghasarraytype" locale="en-US" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
