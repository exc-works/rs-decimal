---
locale: en-GB
source_revision: b5003d3283929da01970193aca2fe128bcc80043185972b8b1e270211629771c
translation_provenance: en-US-reviewed-reuse
api_id: "Decimal::scan"
---
# Decimal::scan

Read a driver-neutral SQL scalar.

```rust
fn scan (& mut self , value : ScanValue < '_ >) -> Result < () , DecimalError >
```

## Parameters, return and normal usage

value: ScanValue; mutates the receiver and returns `Result<(), DecimalError>`/undefined. Choose the variant explicitly: Int64 uses exact bigint; Float32 narrows to binary32; Text/Bytes preserve original decimal digits. SQL text may have one matching pair of surrounding double quotes, which are removed without JSON unescaping. NULL resets the receiver to uninitialized zero. Unsupported types and nonfinite floats return Unmarshal. Other decoding failures retain underlying kinds. All errors preserve receiver state. This adapter requires no database connection.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimal-scan" locale="en-GB" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
