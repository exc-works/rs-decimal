---
locale: en-US
source_revision: 189b6d1ee3eb48467bf982330ec482f99ea7cd30e80f4f3c225f2808c0e03cc3
api_id: "ErrorKind::RoundUnnecessary"
---
# ErrorKind::RoundUnnecessary

Identify RoundUnnecessary failures.

```rust
RoundUnnecessary
```

## Parameters, return and normal usage

Exact rounding was requested but nonzero digits would be discarded.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-roundunnecessary" locale="en-US" />

## Related methods

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
