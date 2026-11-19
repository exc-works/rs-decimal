---
locale: de
source_revision: 1d04e1207bda167aea5c52bc9b525b3aa5208f7c0d21a8e9e4cdf61685f69f69
translation_provenance: en-US-german-reviewed
api_id: "ScanValue"
---
# ScanValue

Eine treiberneutrale typisierte SQL-Eingabe wählen.

```rust
pub enum ScanValue< 'a >
```

## Parameter, Rückgabe und übliche Verwendung

Das Enum unterscheidet NULL, f32, f64, i64, Text, Textbytes und nicht unterstützte Werte. Wählen Sie vor Decimal.scan oder NullDecimal.scan eine explizite Variante. Ausgeliehene Texte/Bytes müssen in Rust nur während des Aufrufs gültig sein; Browser-Bytes hält das Array des Aufrufers und berücksichtigt Änderungen vor scan. Es wird kein Datenbankdienst kontaktiert.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
