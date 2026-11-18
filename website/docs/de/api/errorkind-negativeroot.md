---
locale: de
source_revision: 2c125881cbf08b883eeea1f8f7ddb1472b5bb7418b552e6c5a9a55f4ae4adeb9
translation_provenance: en-US-german-reviewed
api_id: "ErrorKind::NegativeRoot"
---
# ErrorKind::NegativeRoot

NegativeRoot-Fehler erkennen.

```rust
NegativeRoot
```

## Parameter, Rückgabe und übliche Verwendung

Für einen negativen Wert wurde eine gerade Wurzel angefordert. ErrorKind ist eine stabile Kategorie, keine Meldungszeichenfolge. Browser-Enumwerte sind Rust-Variantennamen; konstruierte Fehler verwenden matches(), abgefangene Bridge-Ausnahmen ein kinds-Array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-negativeroot" locale="de" />

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
