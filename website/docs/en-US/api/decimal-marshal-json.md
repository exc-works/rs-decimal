---
locale: en-US
source_revision: ce7a1e6d51ea25aa6e22728a52c66d3bfd202ca324cd22c0d1d0545a351bced2
api_id: "Decimal::marshal_json"
---
# Decimal::marshal_json

Encode Decimal as json bytes.

```rust
fn marshal_json (& self) -> Vec < u8 >
```

## Parameters, return and normal usage

No parameters; returns `Vec<u8>`/Uint8Array containing JSON bytes. Rust requires the json feature; the browser build enables it. Initialized Decimal values encode as quoted canonical strings; default uninitialized zero encodes as JSON null. The returned bytes are an independent copy and reading does not mutate state.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-marshal-json" locale="en-US" />

## Browser boundaries

Decoder input is limited to 8192 bytes; parameter strings are also length-checked before copying into Wasm. Rust rechecks the [decimal resource policy](../resource-policy) on text and decoded JSON strings, so escapes cannot bypass exponent/scale limits. The core owns invalid UTF-8 and JSON/decimal syntax diagnostics. Input and resource errors cannot partially update receivers; moved wrappers cannot be read or mutated.

## Related methods

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
