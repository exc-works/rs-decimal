---
locale: de
source_revision: b6a1d6732343e8fec8b5e4584b880016ecb9ffdc8cf16c927c04637de618fc19
translation_provenance: en-US-german-reviewed
api_id: "ErrorKind::impl PartialOrd"
---
# ErrorKind::impl PartialOrd

Die Rust-Implementierung von PartialOrd verwenden.

```rust
impl PartialOrd for ErrorKind
```

## Parameter, Rückgabe und übliche Verwendung

Ordnet ErrorKind nach Rust-Deklarationsreihenfolge, nicht nach Schweregrad oder alphabetischem Namen. Dies ist native Rust-Trait-Syntax; der Browser stellt sie nicht als Methode bereit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-partialord" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
