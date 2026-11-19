---
locale: en-GB
source_revision: 59bfa6e69fb4720796373509ad0a7f4325fcfd6dbb9c49ba19604b5d730cd4a1
translation_provenance: en-US-reviewed-reuse
api_id: "NullDecimal::unmarshal_bson_value"
---
# NullDecimal::unmarshal_bson_value

Decode a BSON scalar into an existing value.

```rust
fn unmarshal_bson_value (& mut self , kind : u8 , data : & [u8]) -> Result < () , DecimalError >
```

## Parameters, return and normal usage

kind: u8/number, data: &[u8]/Uint8Array; mutates the receiver and returns `Result<(), DecimalError>`/undefined. Rust requires bson. Supports double (1), string (2), null (10), int32 (16), int64 (18) and Decimal128 (19); numeric payloads use BSON little-endian bytes. Pass raw bytes for int64 to avoid JS number precision loss. BSON null clears valid and resets the inner Decimal to uninitialized zero. Non-null successful nullable decoding sets valid=true. Unsupported kinds, truncated data and nonfinite numbers fail atomically. Compatibility details: string decoding skips its declared last byte without checking NUL; bytes beyond the required payload are ignored. Decimal128 is decoded by the Rust core and retains supported decimal precision; output from marshal_bson_value is still a string.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-nulldecimal-unmarshal-bson-value" locale="en-GB" />

## Related methods

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
