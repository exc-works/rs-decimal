---
locale: de
source_revision: bceeb7ede6fc6a0450eec9bb9848cbe43ca4c0d990bf02edf9131f531001c34f
translation_provenance: en-US-german-reviewed
api_id: "DecimalError::impl PartialEq"
---
# DecimalError::impl PartialEq

Die Rust-Implementierung von PartialEq verwenden.

```rust
impl PartialEq for DecimalError
```

## Parameter, Rückgabe und übliche Verwendung

Gleichheit vergleicht vollständige Fehlermeldung/Kategorien oder die exakte Enumvariante. DecimalError-Gleichheit ist strenger als die Übereinstimmung einer Kategorie. Dies ist native Rust-Trait-Syntax; der Browser stellt sie nicht als Methode bereit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-partialeq" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
