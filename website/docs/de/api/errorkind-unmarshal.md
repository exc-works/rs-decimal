---
locale: de
source_revision: c870cbfccba48189ea2c1f73c60ffc0835f379cdb13fc7e1adf9905fa90662b8
translation_provenance: en-US-german-reviewed
api_id: "ErrorKind::Unmarshal"
---
# ErrorKind::Unmarshal

Unmarshal-Fehler erkennen.

```rust
Unmarshal
```

## Parameter, Rückgabe und übliche Verwendung

Decodierung fehlgeschlagen. Eine weitere Kategorie wie InvalidFormat kann die Ursache beschreiben; prüfen Sie Zugehörigkeit, statt nur eine Art anzunehmen. ErrorKind ist eine stabile Kategorie, keine Meldungszeichenfolge. Browser-Enumwerte sind Rust-Variantennamen; konstruierte Fehler verwenden matches(), abgefangene Bridge-Ausnahmen ein kinds-Array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-unmarshal" locale="de" />

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
