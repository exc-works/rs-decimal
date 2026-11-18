---
locale: en-US
source_revision: 38a30bb0b4a307093eafd823ad26d297985a36153b191d7c695a0f68960ea0e6
api_id: "Decimal::marshal_text"
---
# Decimal::marshal_text

Encode Decimal as text bytes.

```rust
fn marshal_text (& self) -> Vec < u8 >
```

## Parameters, return and normal usage

No parameters; returns `Vec<u8>`/Uint8Array containing canonical ASCII decimal text. Fractional trailing zeros are removed. Default Decimal zero prints ASCII 0. The returned bytes are an independent copy and reading does not mutate state.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-marshal-text" locale="en-US" />

## Browser boundaries

Decoder input is limited to 8192 bytes; parameter strings are also length-checked before copying into Wasm. Rust rechecks the [decimal resource policy](../resource-policy) on text and decoded JSON strings, so escapes cannot bypass exponent/scale limits. The core owns invalid UTF-8 and JSON/decimal syntax diagnostics. Input and resource errors cannot partially update receivers; moved wrappers cannot be read or mutated.

## Related methods

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
