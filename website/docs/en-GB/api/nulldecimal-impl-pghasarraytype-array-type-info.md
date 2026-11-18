---
locale: en-GB
source_revision: 1e0fd629af73a5e538f91b272bef72197cbcd10de91d5bebf50a93024747c0fa
translation_provenance: en-US-reviewed-reuse
api_id: "NullDecimal::impl PgHasArrayType::array_type_info"
---
# NullDecimal::impl PgHasArrayType::array_type_info

Use the native SQLx PostgreSQL NUMERIC integration.

```rust
fn array_type_info () -> PgTypeInfo
```

## Normal usage and caveats

Requires the postgres feature and SQLx 0.8 PostgreSQL types. This implementation is native-only: the browser playground does not open database connections. array_type_info() returns NUMERIC[] OID 1231, enabling SQLx array bindings. NullDecimal array elements retain SQL NULL via validity; use native SQLx queries to encode/decode complete arrays.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-nulldecimal-impl-pghasarraytype-array-type-info" locale="en-GB" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
