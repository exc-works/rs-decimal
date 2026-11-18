---
locale: en-GB
source_revision: 8ab6c44a0e4cf327b955869493409ea9bcd447a1bc1451590926853f00b0ca0b
translation_provenance: en-US-reviewed-reuse
api_id: "Decimal::unmarshal_json"
---
# Decimal::unmarshal_json

Decode json input into an existing Decimal.

```rust
fn unmarshal_json (& mut self , bytes : & [u8]) -> Result < () , DecimalError >
```

## Parameters, return and normal usage

bytes: &[u8]/Uint8Array; mutates the receiver and returns Result<(), DecimalError>/undefined. Rust requires the json feature. Pass original UTF-8 JSON bytes: numbers are validated without a float round trip, and strings are unescaped before exact decimal parsing. Do not JSON.parse a numeric token into a JS number first. JSON whitespace is only space/tab/CR/LF. JSON null leaves the entire existing Decimal unchanged, including initialization. Successful non-null parsing initializes the decimal, preserves nonzero trailing scale and normalizes parsed zero to scale zero. Errors preserve the full receiver state. Library decode errors retain Unmarshal and any underlying kinds; resource failures also occur before mutation.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-unmarshal-json" locale="en-GB" />

## Browser boundaries

Decoder input is limited to 8192 bytes; parameter strings are also length-checked before copying into Wasm. Rust rechecks the [decimal resource policy](../resource-policy) on text and decoded JSON strings, so escapes cannot bypass exponent/scale limits. The core owns invalid UTF-8 and JSON/decimal syntax diagnostics. Input and resource errors cannot partially update receivers; moved wrappers cannot be read or mutated.

## Related methods

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
