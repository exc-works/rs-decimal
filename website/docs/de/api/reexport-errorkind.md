---
locale: de
source_revision: 76aa255f8bc8b60ca22e7c6936a048607e56eb1eaa641962626233731b182883
translation_provenance: en-US-german-reviewed
api_id: "reexport::ErrorKind"
---
# reexport::ErrorKind

ErrorKind aus der Crate-Wurzel importieren.

```rust
pub use error::ErrorKind as ErrorKind
```

## Parameter, Rückgabe und übliche Verwendung

rs_decimal::ErrorKind für dasselbe öffentliche Element verwenden. Dies ist ein Importpfad, keine zusätzliche Implementierung oder Browsermethode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-reexport-errorkind" locale="de" />

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
