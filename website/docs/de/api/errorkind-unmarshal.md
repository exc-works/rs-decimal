---
locale: de
source_revision: 274359ec57012f0ef20672a2e83c98afd2d82df0c75391b519022c4925e09ca8
translation_provenance: en-US-german-reviewed
api_id: "ErrorKind::Unmarshal"
---
# ErrorKind::Unmarshal

Unmarshal-Fehler erkennen.

```rust
Unmarshal
```

## Parameter, Rückgabe und übliche Verwendung

Decodierung fehlgeschlagen. Eine weitere Kategorie wie InvalidFormat kann die Ursache beschreiben; prüfen Sie Zugehörigkeit, statt nur eine Art anzunehmen. ErrorKind ist eine stabile Kategorie, keine Meldungszeichenfolge. Browser-Enumwerte sind Rust-Variantennamen; konstruierte Fehler verwenden matches(), abgefangene Bridge-Ausnahmen ein kinds-Array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-unmarshal" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
