---
locale: de
source_revision: 001b8b003c099ab66879c97c5d63d2105fa698bcbc3c15c760670300d17a20a3
translation_provenance: en-US-german-reviewed
api_id: "ErrorKind::impl Eq"
---
# ErrorKind::impl Eq

Die Rust-Implementierung von Eq verwenden.

```rust
impl Eq for ErrorKind
```

## Parameter, Rückgabe und übliche Verwendung

Gleichheit vergleicht vollständige Fehlermeldung/Kategorien oder die exakte Enumvariante. DecimalError-Gleichheit ist strenger als die Übereinstimmung einer Kategorie. Dies ist native Rust-Trait-Syntax; der Browser stellt sie nicht als Methode bereit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-eq" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
