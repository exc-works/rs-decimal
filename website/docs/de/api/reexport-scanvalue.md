---
locale: de
source_revision: 8106ef71942ee657518a42cba33a247697b1ac2ebfcf011160e32c4adeb6e0b5
translation_provenance: en-US-german-reviewed
api_id: "reexport::ScanValue"
---
# reexport::ScanValue

ScanValue aus der Crate-Wurzel importieren.

```rust
pub use sql::ScanValue as ScanValue
```

## Parameter, Rückgabe und übliche Verwendung

rs_decimal::ScanValue für dasselbe öffentliche Element verwenden. Dies ist ein Importpfad, keine zusätzliche Implementierung oder Browsermethode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-reexport-scanvalue" locale="de" />

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
