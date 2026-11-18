---
locale: de
source_revision: eede40b6a84e8fed75b7b014beb0ed7d44fa48b5c63abef14a875a6d37524b2d
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

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
