---
locale: de
source_revision: 0dd0f243213461bb54e73305b48db01cf1b491f68e6951552c4179ef98edf6bf
translation_provenance: en-US-german-reviewed
api_id: "reexport::DecimalError"
---
# reexport::DecimalError

DecimalError aus der Crate-Wurzel importieren.

```rust
pub use error::DecimalError as DecimalError
```

## Parameter, Rückgabe und übliche Verwendung

rs_decimal::DecimalError für dasselbe öffentliche Element verwenden. Dies ist ein Importpfad, keine zusätzliche Implementierung oder Browsermethode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-reexport-decimalerror" locale="de" />

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
