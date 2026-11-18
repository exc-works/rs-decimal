---
locale: en-US
source_revision: f56da33c81a26f95ee4ac8cfce759062606ade6eed7055366c2fe672d142ca01
api_id: "NullDecimal::impl Type < Postgres >::type_info"
---
# NullDecimal::impl Type < Postgres >::type_info

Use the native SQLx PostgreSQL NUMERIC integration.

```rust
fn type_info () -> PgTypeInfo
```

## Normal usage and caveats

Requires the postgres feature and SQLx 0.8 PostgreSQL types. This implementation is native-only: the browser playground does not open database connections. type_info() returns NUMERIC OID 1700. compatible(ty) accepts that exact OID, not float8, integer or arbitrary textual types; no implicit float conversion is promised.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-nulldecimal-impl-type-postgres-type-info" locale="en-US" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
