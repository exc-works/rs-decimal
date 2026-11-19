---
locale: de
source_revision: e2bc3815435c7e993973dc27c3f32515fd48346d012f56633ed09feff4f0aa99
translation_provenance: en-US-german-reviewed
api_id: "ErrorKind::NegativeRoot"
---
# ErrorKind::NegativeRoot

NegativeRoot-Fehler erkennen.

```rust
NegativeRoot
```

## Parameter, Rückgabe und übliche Verwendung

Für einen negativen Wert wurde eine gerade Wurzel angefordert. ErrorKind ist eine stabile Kategorie, keine Meldungszeichenfolge. Browser-Enumwerte sind Rust-Variantennamen; konstruierte Fehler verwenden matches(), abgefangene Bridge-Ausnahmen ein kinds-Array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-negativeroot" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
