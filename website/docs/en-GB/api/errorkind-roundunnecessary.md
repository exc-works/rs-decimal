---
locale: en-GB
source_revision: dc7af33554badb1c95cd11064df3b51faeaca3112afc1b82cf2efcaf4ff1844d
translation_provenance: en-US-reviewed-reuse
api_id: "ErrorKind::RoundUnnecessary"
---
# ErrorKind::RoundUnnecessary

Identify RoundUnnecessary failures.

```rust
RoundUnnecessary
```

## Parameters, return and normal usage

Exact rounding was requested but nonzero digits would be discarded. ErrorKind is a stable category rather than a message string. Browser enum values are Rust variant names; constructed errors use matches(), while caught bridge exceptions expose a kinds array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-roundunnecessary" locale="en-GB" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
