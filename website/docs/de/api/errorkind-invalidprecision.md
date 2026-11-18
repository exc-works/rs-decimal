---
locale: de
source_revision: f8d4a19c8dfdd338ced205f1c85d4c5643f5573e19f1a55cd3492da89573f098
translation_provenance: en-US-german-reviewed
api_id: "ErrorKind::InvalidPrecision"
---
# ErrorKind::InvalidPrecision

InvalidPrecision-Fehler erkennen.

```rust
InvalidPrecision
```

## Parameter, Rückgabe und übliche Verwendung

Die angeforderte Präzision ist ungültig. ErrorKind ist eine stabile Kategorie, keine Meldungszeichenfolge. Browser-Enumwerte sind Rust-Variantennamen; konstruierte Fehler verwenden matches(), abgefangene Bridge-Ausnahmen ein kinds-Array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidprecision" locale="de" />

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
