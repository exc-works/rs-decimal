---
locale: en-GB
source_revision: 274359ec57012f0ef20672a2e83c98afd2d82df0c75391b519022c4925e09ca8
translation_provenance: en-US-reviewed-reuse
api_id: "ErrorKind::Unmarshal"
---
# ErrorKind::Unmarshal

Identify Unmarshal failures.

```rust
Unmarshal
```

## Parameters, return and normal usage

Decoding failed. Another category such as InvalidFormat may describe its cause; test membership rather than assuming one kind. ErrorKind is a stable category rather than a message string. Browser enum values are Rust variant names; constructed errors use matches(), while caught bridge exceptions expose a kinds array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-unmarshal" locale="en-GB" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
