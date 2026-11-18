---
locale: en-US
source_revision: f3551e314b31f667d7ca138dd63f4da03440121f2536e11a0e604ede2dbe9e2b
api_id: "NullDecimal::impl Encode < '_ , Postgres >"
---
# NullDecimal::impl Encode < '_ , Postgres >

Use the native SQLx PostgreSQL NUMERIC integration.

```rust
impl Encode < '_ , Postgres > for NullDecimal 
```

## Normal usage and caveats

Requires the postgres feature and SQLx 0.8 PostgreSQL types. This implementation is native-only: the browser playground does not open database connections. encode_by_ref(&self, &mut PgArgumentBuffer) returns `Result<IsNull, BoxDynError>`. Borrowing preserves the value; bytes append to the existing argument buffer only after successful encoding. Invalid nullable values return IsNull::Yes without bytes. Valid values encode the inner Decimal, even if its zero was uninitialized. The binary format uses big-endian base-10000 groups, preserves display scale, and has no float/BigDecimal intermediary. Scale above 16383 or more than 131072 integer digits is rejected; encoding errors preserve the destination buffer.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-nulldecimal-impl-encode-postgres" locale="en-US" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
