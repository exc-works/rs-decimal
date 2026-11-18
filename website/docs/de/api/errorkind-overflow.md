---
locale: de
source_revision: b5e19b2f97fd7e9d8a942453bc1d3336bd82a5f0e33e943baa1dc21eb3e5ac5f
translation_provenance: en-US-german-reviewed
api_id: "ErrorKind::Overflow"
---
# ErrorKind::Overflow

Overflow-Fehler erkennen.

```rust
Overflow
```

## Parameter, Rückgabe und übliche Verwendung

Ein Koeffizient, eine Skala oder Operation überschreitet den unterstützten Kernbereich. Browser-Ressourcenfehler sind separat. ErrorKind ist eine stabile Kategorie, keine Meldungszeichenfolge. Browser-Enumwerte sind Rust-Variantennamen; konstruierte Fehler verwenden matches(), abgefangene Bridge-Ausnahmen ein kinds-Array.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-overflow" locale="de" />

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
