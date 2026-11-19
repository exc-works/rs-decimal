---
locale: de
source_revision: 6eb93b97fb5de84f7620ea6f0aa7d8d8ce7bbe38e3bab65b24ca028dc1ed97cd
translation_provenance: en-US-german-reviewed
api_id: "ErrorKind::Overflow"
---
# ErrorKind::Overflow

Overflow-Fehler erkennen.

```rust
Overflow
```

## Parameter, Rückgabe und übliche Verwendung

Ein Koeffizient, eine Skala oder Operation überschreitet den unterstützten Kernbereich. Browser-Ressourcenfehler sind separat. ErrorKind ist eine stabile Kategorie, keine Meldungszeichenfolge. Browser-Enumwerte sind Rust-Variantennamen; konstruierte Fehler verwenden matches(), abgefangene Bridge-Ausnahmen ein kinds-Array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-overflow" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
