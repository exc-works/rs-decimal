---
locale: de
source_revision: 368adfaeea00a469d274e0e87fc3e84f66ca2848d32cfd8759858b2bac06fe99
translation_provenance: en-US-german-reviewed
api_id: "ErrorKind::Overflow"
---
# ErrorKind::Overflow

Overflow-Fehler erkennen.

```rust
Overflow
```

## Parameter, Rückgabe und übliche Verwendung

Ein Koeffizient, eine Skala oder Operation überschreitet den unterstützten Kernbereich. ErrorKind ist eine stabile Kategorie, keine Meldungszeichenfolge. Browser-Enumwerte sind Rust-Variantennamen; konstruierte Fehler verwenden matches(), abgefangene Bridge-Ausnahmen ein kinds-Array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-overflow" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
