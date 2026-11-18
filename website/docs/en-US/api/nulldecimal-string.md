---
locale: en-US
source_revision: 753b858d717076ef46e98ae0c0b43697fb31b0651286396a37e7378758f67207
api_id: "NullDecimal::string"
---
# NullDecimal::string

Read nullable canonical text.

```rust
fn string (& self) -> String
```

## Parameters, return and normal usage

No parameters; returns String/string. Valid values use the inner Decimal canonical text, removing fractional trailing zeros. Invalid values return the literal null, regardless of hidden inner data. This differs from marshal_text, which returns empty bytes for invalid values. Reading does not change validity or initialization.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-string" locale="en-US" />

## Browser boundaries

Decoder input is limited to 8192 bytes; parameter strings are also length-checked before copying into Wasm. Rust rechecks the [decimal resource policy](../resource-policy) on text and decoded JSON strings, so escapes cannot bypass exponent/scale limits. The core owns invalid UTF-8 and JSON/decimal syntax diagnostics. Input and resource errors cannot partially update receivers; moved wrappers cannot be read or mutated.

## Related methods

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
