---
locale: en-US
source_revision: c4d07db94cb802a1abe829e9593804f52c2de0bb0ad545606bf47e6e769fde05
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

<MethodExample id="text-json-decimal-unmarshal-param" locale="en-US" />

## Related methods

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
