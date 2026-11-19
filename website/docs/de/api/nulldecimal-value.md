---
locale: de
source_revision: cd65ee6368cc890889ec3fceb39f3b12814d9f2d0fe811e6c57c730bc5d81b5b
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

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
