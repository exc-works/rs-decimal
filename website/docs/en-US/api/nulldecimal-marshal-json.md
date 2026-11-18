---
locale: en-US
source_revision: ce07dfd04d96401a00570a69b98dc03abf2e70bda05348ea2e8e6bff0225c443
api_id: "NullDecimal::marshal_json"
---
# NullDecimal::marshal_json

Encode NullDecimal as json bytes.

```rust
fn marshal_json (& self) -> Vec < u8 >
```

## Parameters, return and normal usage

No parameters; returns `Vec<u8>`/Uint8Array containing JSON bytes. Rust requires the json feature; the browser build enables it. Invalid nullable values encode as JSON null; valid values are quoted canonical decimals, even when the inner Decimal is uninitialized. The returned bytes are an independent copy and reading does not mutate state.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-marshal-json" locale="en-US" />

## Browser boundaries

Decoder input is limited to 8192 bytes; parameter strings are also length-checked before copying into Wasm. Rust rechecks the [decimal resource policy](../resource-policy) on text and decoded JSON strings, so escapes cannot bypass exponent/scale limits. The core owns invalid UTF-8 and JSON/decimal syntax diagnostics. Input and resource errors cannot partially update receivers; moved wrappers cannot be read or mutated.

## Related methods

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
