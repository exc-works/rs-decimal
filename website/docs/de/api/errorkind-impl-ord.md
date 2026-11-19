---
locale: de
source_revision: 77c1d26d73dd77cab1d0c87534f9e5ec0b5d57b39f76087cd3ef2b288f8c6fae
translation_provenance: en-US-german-reviewed
api_id: "ErrorKind::impl Ord"
---
# ErrorKind::impl Ord

Die Rust-Implementierung von Ord verwenden.

```rust
impl Ord for ErrorKind
```

## Parameter, Rückgabe und übliche Verwendung

Ordnet ErrorKind nach Rust-Deklarationsreihenfolge, nicht nach Schweregrad oder alphabetischem Namen. Dies ist native Rust-Trait-Syntax; der Browser stellt sie nicht als Methode bereit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-ord" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
