---
locale: en-US
source_revision: b07552403b149d5a66b24c1aa4c0547e8259f4d124ac401ca1161c70929a6aa6
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

<MethodExample id="binary-bson-nulldecimal-marshal-bson-value" locale="en-US" />

## Browser boundaries

Input must be Uint8Array, and kind an integer in 0–255. Inputs are capped at 65536 bytes before copying to Wasm. Rust checks binary scale up to 1000 and actual coefficient size up to 16384 bits. BSON strings follow the 8192-byte text limit, and Decimal128 exponent expansion is also [resource-checked](../resource-policy). These limits can be narrower than the native library. Checks precede receiver mutation; core and resource errors remain distinct, and moved handles cannot be reused.

## Related methods

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
