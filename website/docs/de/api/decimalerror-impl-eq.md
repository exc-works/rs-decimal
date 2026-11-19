---
locale: de
source_revision: cbf84996c156999f0cfb9cac49a1135ecf0d19ca8aba3abbedbed0acff443c0a
translation_provenance: en-US-german-reviewed
api_id: "DecimalError::impl Eq"
---
# DecimalError::impl Eq

Die Rust-Implementierung von Eq verwenden.

```rust
impl Eq for DecimalError
```

## Parameter, Rückgabe und übliche Verwendung

Gleichheit vergleicht vollständige Fehlermeldung/Kategorien oder die exakte Enumvariante. DecimalError-Gleichheit ist strenger als die Übereinstimmung einer Kategorie. Dies ist native Rust-Trait-Syntax; der Browser stellt sie nicht als Methode bereit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-eq" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
