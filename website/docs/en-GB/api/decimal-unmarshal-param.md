---
locale: en-GB
source_revision: 00df85dbc78a408d1e3a1357be14942f9ab79794d46b2e7624e5bd2c1ceeaef3
translation_provenance: en-US-reviewed-reuse
api_id: "Decimal::unmarshal_param"
---
# Decimal::unmarshal_param

Decode param input into an existing Decimal.

```rust
fn unmarshal_param (& mut self , param : & str) -> Result < () , DecimalError >
```

## Parameters, return and normal usage

param: &str/string; mutates the receiver and returns Result<(), DecimalError>/undefined. Parses decimal text, including trimmed whitespace and exponent syntax. Empty text and literal null are invalid decimal text; use nullable decoding when emptiness means no value. Successful non-null parsing initializes the decimal, preserves nonzero trailing scale and normalizes parsed zero to scale zero. Errors preserve the full receiver state. Library decode errors retain Unmarshal and any underlying kinds; resource failures also occur before mutation.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-unmarshal-param" locale="en-GB" />

## Browser boundaries

Decoder input is limited to 8192 bytes; parameter strings are also length-checked before copying into Wasm. Rust rechecks the [decimal resource policy](../resource-policy) on text and decoded JSON strings, so escapes cannot bypass exponent/scale limits. The core owns invalid UTF-8 and JSON/decimal syntax diagnostics. Input and resource errors cannot partially update receivers; moved wrappers cannot be read or mutated.

## Related methods

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
