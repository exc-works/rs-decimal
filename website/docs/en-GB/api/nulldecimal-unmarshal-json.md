---
locale: en-GB
source_revision: d67309b22e8bef25fa612b0b77506dc49f39a3b10f47781eb6058acd17162dd3
translation_provenance: en-US-reviewed-reuse
api_id: "NullDecimal::unmarshal_json"
---
# NullDecimal::unmarshal_json

Decode json input into an existing NullDecimal.

```rust
fn unmarshal_json (& mut self , bytes : & [u8]) -> Result < () , DecimalError >
```

## Parameters, return and normal usage

bytes: &[u8]/Uint8Array; mutates the receiver and returns Result<(), DecimalError>/undefined. Rust requires the json feature. Pass original UTF-8 JSON bytes: numbers are validated without a float round trip, and strings are unescaped before exact decimal parsing. Do not JSON.parse a numeric token into a JS number first. JSON whitespace is only space/tab/CR/LF. JSON null clears validity and resets the inner Decimal to uninitialized zero. Successful non-null parsing initializes the decimal, preserves nonzero trailing scale and normalizes parsed zero to scale zero. It sets valid=true. Errors preserve the full receiver state. Library decode errors retain Unmarshal and any underlying kinds; resource failures also occur before mutation.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-unmarshal-json" locale="en-GB" />

## Related methods

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
