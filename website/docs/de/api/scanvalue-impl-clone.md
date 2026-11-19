---
locale: de
source_revision: 11ee599a7357841e984cdaecadf4da635302996e14ae33191c323e57e59262ff
translation_provenance: en-US-german-reviewed
api_id: "ScanValue::impl Clone"
---
# ScanValue::impl Clone

Die Rust-Implementierung von Clone verwenden.

```rust
impl Clone for ScanValue
```

## Parameter, Rückgabe und übliche Verwendung

Clone erhält den vollständigen Wert; ausgeliehene ScanValue-Slices verweisen weiterhin auf die ursprünglichen Daten. Dies ist native Rust-Trait-Syntax; der Browser stellt sie nicht als Methode bereit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-impl-clone" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
