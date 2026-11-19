---
locale: de
source_revision: b276bcddff5985f20e7905654e4b3cef6f352b3cfa139550a92b9425bd777e62
translation_provenance: en-US-german-reviewed
api_id: "ScanValue::Float64"
---
# ScanValue::Float64

Die SQL-Eingabevariante Float64 wählen.

```rust
Float64 (f64)
```

## Parameter, Rückgabe und übliche Verwendung

binary64 transportieren. Endliche Werte verwenden die kürzeste Dezimalkonvertierung des Kerns; NaN und Unendlichkeiten liefern Unmarshal. Bereits gerundete JS-Zahlen stellen ursprüngliche Ziffern nicht wieder her. Varianten werden von scan ausgeliehen und können innerhalb eines Browserlaufs wiederverwendet werden; erfundene Werte oder solche aus fremden Läufen werden abgewiesen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-float64" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
