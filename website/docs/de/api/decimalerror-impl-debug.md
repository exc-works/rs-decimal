---
locale: de
source_revision: 8ab3953570c1516f55bc48b7c251f1cfbcd5ae142c6f5206aaaaa5635e651b35
translation_provenance: en-US-german-reviewed
api_id: "DecimalError::impl Debug"
---
# DecimalError::impl Debug

Die Rust-Implementierung von Debug verwenden.

```rust
impl Debug for DecimalError
```

## Parameter, Rückgabe und übliche Verwendung

Debug liefert Diagnosetext, keine stabile Übertragungsserialisierung. Dies ist native Rust-Trait-Syntax; der Browser stellt sie nicht als Methode bereit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-debug" locale="de" />

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
