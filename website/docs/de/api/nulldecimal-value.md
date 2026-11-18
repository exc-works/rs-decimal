---
locale: de
source_revision: 79298c474720d5ca067e6e36ecdefe721611687b58bab5fff1d35e1889208bcd
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::value"
---
# NullDecimal::value

Den kanonischen SQL-Wert erhalten.

```rust
fn value (& self) -> Option < String >
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert `Option<String>`/string | null. Ungültige nullable-Werte liefern unabhängig von verborgenen inneren Daten None/null. Gültige Werte liefern kanonischen Dezimaltext, einschließlich "0" bei innerer nicht initialisierter Null. Entfernt nachgestellte Nachkommanullen ohne Änderung von Skala oder Initialisierung. Der Text ist ein Skalarwert, kein in Anführungszeichen gesetztes SQL-Abfragefragment.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-nulldecimal-value" locale="de" />

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
