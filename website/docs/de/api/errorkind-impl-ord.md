---
locale: de
source_revision: 93c9a590df79625539c288c9f6a42d27aa1dba571b28363b98f9ec4642104e16
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

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
