---
locale: en-US
source_revision: 7dc5d9b6c0f5030d430dc8fa4c821516f6b5bfe8979234b7de2b0cf0b1d59588
api_id: "Decimal::impl Type < Postgres >"
---
# Decimal::impl Type < Postgres >

Use the native SQLx PostgreSQL NUMERIC integration.

```rust
impl Type < Postgres > for Decimal 
```

## Normal usage and caveats

Requires the postgres feature and SQLx 0.8 PostgreSQL types. This implementation is native-only: the browser playground does not open database connections. type_info() returns NUMERIC OID 1700. compatible(ty) accepts that exact OID, not float8, integer or arbitrary textual types; no implicit float conversion is promised.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-type-postgres" locale="en-US" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
