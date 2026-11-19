---
locale: de
source_revision: c456812ef50493713a2bbd81d6c57db5a0f730099b5878e83b3d4f358245d822
translation_provenance: en-US-german-reviewed
api_id: "ErrorKind::InvalidLog"
---
# ErrorKind::InvalidLog

InvalidLog-Fehler erkennen.

```rust
InvalidLog
```

## Parameter, Rückgabe und übliche Verwendung

Eine Logarithmuseingabe ist nichtpositiv. ErrorKind ist eine stabile Kategorie, keine Meldungszeichenfolge. Browser-Enumwerte sind Rust-Variantennamen; konstruierte Fehler verwenden matches(), abgefangene Bridge-Ausnahmen ein kinds-Array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidlog" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
