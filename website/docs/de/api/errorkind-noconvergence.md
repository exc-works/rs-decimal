---
locale: de
source_revision: aeb152b6bffb5f406cc2fbe512dd2449186121fa1cd03efbf5d74f0d3891aed1
translation_provenance: en-US-german-reviewed
api_id: "ErrorKind::NoConvergence"
---
# ErrorKind::NoConvergence

NoConvergence-Fehler erkennen.

```rust
NoConvergence
```

## Parameter, Rückgabe und übliche Verwendung

Der begrenzte Algorithmus konnte kein gerundetes Ergebnis zertifizieren; das bedeutet nicht, dass kein mathematisches Ergebnis existiert. ErrorKind ist eine stabile Kategorie, keine Meldungszeichenfolge. Browser-Enumwerte sind Rust-Variantennamen; konstruierte Fehler verwenden matches(), abgefangene Bridge-Ausnahmen ein kinds-Array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-noconvergence" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
