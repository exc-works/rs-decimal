---
locale: de
source_revision: e14d22e2fbe70e02eda750145ed5b1a70252e171e63aeaafcfc01281be24e818
translation_provenance: en-US-german-reviewed
api_id: "ErrorKind::InvalidArgument"
---
# ErrorKind::InvalidArgument

InvalidArgument-Fehler erkennen.

```rust
InvalidArgument
```

## Parameter, Rückgabe und übliche Verwendung

Ein Argument, eine Validierungsregel oder Nachrichtenvorlage ist ungültig. ErrorKind ist eine stabile Kategorie, keine Meldungszeichenfolge. Browser-Enumwerte sind Rust-Variantennamen; konstruierte Fehler verwenden matches(), abgefangene Bridge-Ausnahmen ein kinds-Array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidargument" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
