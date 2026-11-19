---
locale: de
source_revision: 8064cdb13ca114f6f1b29d727ed53f33f638f79926b35e606248ced643f822e9
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

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
