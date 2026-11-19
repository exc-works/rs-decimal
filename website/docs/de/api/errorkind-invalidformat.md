---
locale: de
source_revision: 72523519c8ccf1aad4c8baebb6ac1ef42e8070b9cc7f7f72b600ff77781eaf4e
translation_provenance: en-US-german-reviewed
api_id: "ErrorKind::InvalidFormat"
---
# ErrorKind::InvalidFormat

InvalidFormat-Fehler erkennen.

```rust
InvalidFormat
```

## Parameter, Rückgabe und übliche Verwendung

Die Eingabe ist keine unterstützte endliche Dezimaldarstellung. ErrorKind ist eine stabile Kategorie, keine Meldungszeichenfolge. Browser-Enumwerte sind Rust-Variantennamen; konstruierte Fehler verwenden matches(), abgefangene Bridge-Ausnahmen ein kinds-Array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidformat" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
