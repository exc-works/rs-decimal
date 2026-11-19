---
locale: en-GB
source_revision: 9fa4dc63685c8b7863c54238c3b1bc5377c717d45438360151595cb826deb45d
translation_provenance: en-US-reviewed-reuse
api_id: "NullDecimal::marshal_bson_value"
---
# NullDecimal::marshal_bson_value

Encode a BSON type byte and scalar payload.

```rust
fn marshal_bson_value (& self) -> Result < (u8 , Vec < u8 >) , DecimalError >
```

## Parameters, return and normal usage

No parameters; returns `Result<(u8, Vec<u8>), DecimalError>`/[number, Uint8Array]. Rust requires the bson feature, enabled in the browser build. The tuple contains a raw BSON scalar, not a complete BSON document. Valid nullable values encode as BSON strings, including valid values with an uninitialized inner zero. Invalid values encode as type 10 with empty payload. The string payload has a little-endian i32 byte length including its terminating NUL, followed by canonical decimal text without fractional trailing zeros. Returned payload bytes are an independent copy; encoding does not alter initialization or validity.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-nulldecimal-marshal-bson-value" locale="en-GB" />

## Related methods

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
