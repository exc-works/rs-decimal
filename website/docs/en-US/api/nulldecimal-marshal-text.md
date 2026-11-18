---
locale: en-US
source_revision: d7c6490115eacc09ab0edd5dff3653c7f3971bc72919c598a24d41b9dd4bd07a
api_id: "NullDecimal::marshal_text"
---
# NullDecimal::marshal_text

Encode NullDecimal as text bytes.

```rust
fn marshal_text (& self) -> Vec < u8 >
```

## Parameters, return and normal usage

No parameters; returns `Vec<u8>`/Uint8Array containing canonical ASCII decimal text. Valid values encode canonical decimal text; invalid values encode empty bytes, not the word null. The returned bytes are an independent copy and reading does not mutate state.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-marshal-text" locale="en-US" />

## Browser boundaries

Decoder input is limited to 8192 bytes; parameter strings are also length-checked before copying into Wasm. Rust rechecks the [decimal resource policy](../resource-policy) on text and decoded JSON strings, so escapes cannot bypass exponent/scale limits. The core owns invalid UTF-8 and JSON/decimal syntax diagnostics. Input and resource errors cannot partially update receivers; moved wrappers cannot be read or mutated.

## Related methods

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
