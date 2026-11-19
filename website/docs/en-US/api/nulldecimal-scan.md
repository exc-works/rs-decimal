---
locale: en-US
source_revision: 3c6ebaee1d6c2d15a7801caee0a8597128eea9b110eb2748ac00d60cf69e23fd
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

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
