---
locale: en-US
source_revision: 44afbeff0d59cbde33e83e7f33c7e22c9903950f7ecf43d6f8230c6cde4b2ec4
api_id: "Decimal::marshal_bson_value"
---
# Decimal::marshal_bson_value

Encode a BSON type byte and scalar payload.

```rust
fn marshal_bson_value (& self) -> Result < (u8 , Vec < u8 >) , DecimalError >
```

## Parameters, return and normal usage

No parameters; returns `Result<(u8, Vec<u8>), DecimalError>`/[number, Uint8Array]. Rust requires the bson feature, enabled in the browser build. The tuple contains a raw BSON scalar, not a complete BSON document. Initialized values encode as BSON strings (type 2), not Decimal128. Uninitialized zero encodes as type 10 with empty payload. The string payload has a little-endian i32 byte length including its terminating NUL, followed by canonical decimal text without fractional trailing zeros. Returned payload bytes are an independent copy; encoding does not alter initialization or validity.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-marshal-bson-value" locale="en-US" />

## Related methods

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
