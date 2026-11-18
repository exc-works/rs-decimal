---
locale: en-US
source_revision: 329a8011aecf030884cb84a14d70c08abd0f17ed41927f957f56a638648169ac
api_id: "NullDecimal::impl PgHasArrayType"
---
# NullDecimal::impl PgHasArrayType

Use the native SQLx PostgreSQL NUMERIC integration.

```rust
impl PgHasArrayType for NullDecimal 
```

## Normal usage and caveats

Requires the postgres feature and SQLx 0.8 PostgreSQL types. This implementation is native-only: the browser playground does not open database connections. array_type_info() returns NUMERIC[] OID 1231, enabling SQLx array bindings. NullDecimal array elements retain SQL NULL via validity; use native SQLx queries to encode/decode complete arrays.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-nulldecimal-impl-pghasarraytype" locale="en-US" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
