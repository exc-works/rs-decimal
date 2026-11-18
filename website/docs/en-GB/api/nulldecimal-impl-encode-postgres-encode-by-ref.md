---
locale: en-GB
source_revision: f24ab15a12ac3cc5edffa3bb6354648be7a4944a94929ac82037f54fe89c586d
translation_provenance: en-US-reviewed-reuse
api_id: "NullDecimal::impl Encode < '_ , Postgres >::encode_by_ref"
---
# NullDecimal::impl Encode < '_ , Postgres >::encode_by_ref

Use the native SQLx PostgreSQL NUMERIC integration.

```rust
fn encode_by_ref (& self , buf : & mut PgArgumentBuffer) -> Result < IsNull , BoxDynError >
```

## Normal usage and caveats

Requires the postgres feature and SQLx 0.8 PostgreSQL types. This implementation is native-only: the browser playground does not open database connections. encode_by_ref(&self, &mut PgArgumentBuffer) returns `Result<IsNull, BoxDynError>`. Borrowing preserves the value; bytes append to the existing argument buffer only after successful encoding. Invalid nullable values return IsNull::Yes without bytes. Valid values encode the inner Decimal, even if its zero was uninitialized. The binary format uses big-endian base-10000 groups, preserves display scale, and has no float/BigDecimal intermediary. Scale above 16383 or more than 131072 integer digits is rejected; encoding errors preserve the destination buffer.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-nulldecimal-impl-encode-postgres-encode-by-ref" locale="en-GB" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
