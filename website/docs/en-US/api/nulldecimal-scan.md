---
locale: en-US
source_revision: 559007df6751f5417671f4f4b9a33c4ae6759ecf82dbd4fbeb7d9f0c40061e1a
api_id: "NullDecimal::scan"
---
# NullDecimal::scan

Read a driver-neutral SQL scalar.

```rust
fn scan (& mut self , value : ScanValue < '_ >) -> Result < () , DecimalError >
```

## Parameters, return and normal usage

value: ScanValue; mutates the receiver and returns `Result<(), DecimalError>`/undefined. Choose the variant explicitly: Int64 uses exact bigint; Float32 narrows to binary32; Text/Bytes preserve original decimal digits. SQL text may have one matching pair of surrounding double quotes, which are removed without JSON unescaping. NULL clears validity and resets inner Decimal to uninitialized zero; successful non-null input sets valid=true. Unsupported types and nonfinite floats return Unmarshal. Other decoding failures retain underlying kinds. All errors preserve receiver state. This adapter requires no database connection.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-nulldecimal-scan" locale="en-US" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
