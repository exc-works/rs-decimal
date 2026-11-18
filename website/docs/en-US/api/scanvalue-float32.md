---
locale: en-US
source_revision: dbb045138dac94f7b2691e8576e6af500d5ead689869256e178b5069283ab80d
api_id: "ScanValue::Float32"
---
# ScanValue::Float32

Select the Float32 SQL input variant.

```rust
Float32 (f32)
```

## Parameters, return and normal usage

Carry binary32. The browser accepts number and Rust narrows it to f32 before scanning. NaN, infinities and narrowing overflow fail with Unmarshal. Variants are borrowed by scan and can be reused within one browser run; fabricated or foreign-run values are rejected.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-float32" locale="en-US" />

## Browser boundaries

SQL text/byte input and error messages are capped at 8192 UTF-8 bytes. SQL text is [resource-checked](../resource-policy) after removing outer quotes, so quoting cannot bypass exponent limits. Borrowed calls preserve inputs; consumed or disposed handles cannot be reused. Rust trait/import examples run in native tests and do not claim browser method support.

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
