---
locale: en-US
source_revision: b2646b52b58adc30058be6fedbac290221f3b2b947946b06b25fe8783f697afc
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

<MethodExample id="text-json-decimal-unmarshal-json" locale="en-US" />

## Related methods

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
