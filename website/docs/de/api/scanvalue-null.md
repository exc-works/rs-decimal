---
locale: de
source_revision: 17f954a2ac80c90ce4a20dd8e36b0981b73e6748171ba829abe55cdfdc69b2b3
translation_provenance: en-US-german-reviewed
api_id: "ScanValue::Null"
---
# ScanValue::Null

Die SQL-Eingabevariante Null wählen.

```rust
Null
```

## Parameter, Rückgabe und übliche Verwendung

SQL NULL darstellen; Scannen setzt Decimal auf nicht initialisierte Null zurück und leert NullDecimal. Varianten werden von scan ausgeliehen und können innerhalb eines Browserlaufs wiederverwendet werden; erfundene Werte oder solche aus fremden Läufen werden abgewiesen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-null" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
