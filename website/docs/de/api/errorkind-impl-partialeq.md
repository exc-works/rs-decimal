---
locale: de
source_revision: 5ee0e7b51aa068361991bd8a8b6016ae9f25f271ec57ec1d0990b39445f68b64
translation_provenance: en-US-german-reviewed
api_id: "ErrorKind::impl PartialEq"
---
# ErrorKind::impl PartialEq

Die Rust-Implementierung von PartialEq verwenden.

```rust
impl PartialEq for ErrorKind
```

## Parameter, Rückgabe und übliche Verwendung

Gleichheit vergleicht vollständige Fehlermeldung/Kategorien oder die exakte Enumvariante. DecimalError-Gleichheit ist strenger als die Übereinstimmung einer Kategorie. Dies ist native Rust-Trait-Syntax; der Browser stellt sie nicht als Methode bereit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-partialeq" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
