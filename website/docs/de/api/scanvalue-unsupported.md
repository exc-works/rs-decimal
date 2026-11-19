---
locale: de
source_revision: 78277d1082dac8a98f79b7018413113c09afb0c224a6c82ae6c179432b0517f1
translation_provenance: en-US-german-reviewed
api_id: "ScanValue::Unsupported"
---
# ScanValue::Unsupported

Die SQL-Eingabevariante Unsupported wählen.

```rust
Unsupported
```

## Parameter, Rückgabe und übliche Verwendung

Nicht unterstützte Treibereingabe ohne automatische Konvertierung darstellen. scan liefert Unmarshal und erhält den vorhandenen Wert. Varianten werden von scan ausgeliehen und können innerhalb eines Browserlaufs wiederverwendet werden; erfundene Werte oder solche aus fremden Läufen werden abgewiesen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-unsupported" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
