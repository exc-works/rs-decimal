---
locale: de
source_revision: ab8ea6bf03391270a33235fc649739ef25b1cdab822e201005d40879c5be9299
translation_provenance: en-US-german-reviewed
api_id: "Decimal::value"
---
# Decimal::value

Den kanonischen SQL-Wert erhalten.

```rust
fn value (& self) -> String
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert String/string. Selbst nicht initialisierter Decimal liefert nicht-NULL "0"; verwenden Sie NullDecimal für SQL NULL. Entfernt nachgestellte Nachkommanullen ohne Änderung von Skala oder Initialisierung. Der Text ist ein Skalarwert, kein in Anführungszeichen gesetztes SQL-Abfragefragment.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimal-value" locale="de" />

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
