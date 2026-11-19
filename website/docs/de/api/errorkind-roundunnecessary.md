---
locale: de
source_revision: dc7af33554badb1c95cd11064df3b51faeaca3112afc1b82cf2efcaf4ff1844d
translation_provenance: en-US-german-reviewed
api_id: "ErrorKind::RoundUnnecessary"
---
# ErrorKind::RoundUnnecessary

RoundUnnecessary-Fehler erkennen.

```rust
RoundUnnecessary
```

## Parameter, Rückgabe und übliche Verwendung

Exakte Rundung wurde verlangt, aber Ziffern ungleich null müssten verworfen werden. ErrorKind ist eine stabile Kategorie, keine Meldungszeichenfolge. Browser-Enumwerte sind Rust-Variantennamen; konstruierte Fehler verwenden matches(), abgefangene Bridge-Ausnahmen ein kinds-Array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-roundunnecessary" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
