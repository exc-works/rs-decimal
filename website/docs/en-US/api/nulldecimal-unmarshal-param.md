---
locale: en-US
source_revision: 098847ed019375f939ef5ba1a900bfb15c1c278feadb8bae9f85384666b02bf1
api_id: "NullDecimal::unmarshal_param"
---
# NullDecimal::unmarshal_param

Decode param input into an existing NullDecimal.

```rust
fn unmarshal_param (& mut self , param : & str) -> Result < () , DecimalError >
```

## Parameters, return and normal usage

param: &str/string; mutates the receiver and returns Result<(), DecimalError>/undefined. Parses decimal text, including trimmed whitespace and exponent syntax. Empty input clears validity and resets the inner Decimal; literal null is invalid decimal text. Successful non-null parsing initializes the decimal, preserves nonzero trailing scale and normalizes parsed zero to scale zero. It sets valid=true. Errors preserve the full receiver state. Library decode errors retain Unmarshal and any underlying kinds; resource failures also occur before mutation.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-unmarshal-param" locale="en-US" />

## Browser boundaries

Decoder input is limited to 8192 bytes; parameter strings are also length-checked before copying into Wasm. Rust rechecks the [decimal resource policy](../resource-policy) on text and decoded JSON strings, so escapes cannot bypass exponent/scale limits. The core owns invalid UTF-8 and JSON/decimal syntax diagnostics. Input and resource errors cannot partially update receivers; moved wrappers cannot be read or mutated.

## Related methods

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
