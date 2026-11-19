---
locale: de
source_revision: 189b6d1ee3eb48467bf982330ec482f99ea7cd30e80f4f3c225f2808c0e03cc3
translation_provenance: en-US-german-reviewed
api_id: "ErrorKind::RoundUnnecessary"
---
# ErrorKind::RoundUnnecessary

RoundUnnecessary-Fehler erkennen.

```rust
RoundUnnecessary
```

## Parameter, Rückgabe und übliche Verwendung

Exakte Rundung wurde verlangt, aber Ziffern ungleich null müssten verworfen werden.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-roundunnecessary" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
