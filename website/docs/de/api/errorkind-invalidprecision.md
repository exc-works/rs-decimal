---
locale: de
source_revision: 68b84662c55e7239d15b1fe54a5a2076eb45cea41916719d90f7dbe8ab9e7150
translation_provenance: en-US-german-reviewed
api_id: "ErrorKind::InvalidPrecision"
---
# ErrorKind::InvalidPrecision

InvalidPrecision-Fehler erkennen.

```rust
InvalidPrecision
```

## Parameter, Rückgabe und übliche Verwendung

Die angeforderte Präzision ist ungültig. ErrorKind ist eine stabile Kategorie, keine Meldungszeichenfolge. Browser-Enumwerte sind Rust-Variantennamen; konstruierte Fehler verwenden matches(), abgefangene Bridge-Ausnahmen ein kinds-Array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidprecision" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
