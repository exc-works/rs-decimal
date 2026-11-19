---
locale: en-GB
source_revision: 6fa89e7d9fd5a43bce6d83be2495a38047592a77e9611dcd781da000b62e4862
translation_provenance: en-US-reviewed-reuse
api_id: "Decimal::unmarshal_text"
---
# Decimal::unmarshal_text

Decode text input into an existing Decimal.

```rust
fn unmarshal_text (& mut self , text : & [u8]) -> Result < () , DecimalError >
```

## Parameters, return and normal usage

bytes: &[u8]/Uint8Array; mutates the receiver and returns Result<(), DecimalError>/undefined. Parses decimal text, including trimmed whitespace and exponent syntax. Empty text and literal null are invalid decimal text; use nullable decoding when emptiness means no value. Successful non-null parsing initializes the decimal, preserves nonzero trailing scale and normalizes parsed zero to scale zero. Errors preserve the full receiver state. Library decode errors retain Unmarshal and any underlying kinds; resource failures also occur before mutation.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-unmarshal-text" locale="en-GB" />

## Related methods

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
