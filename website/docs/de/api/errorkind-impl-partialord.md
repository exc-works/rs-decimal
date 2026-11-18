---
locale: de
source_revision: cd40a54006cbf363c2c029f006d9d975f79108b87d1ec7bdd4a2b4ab26efe940
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

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
