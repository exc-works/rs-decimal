---
locale: de
source_revision: 15c94f02827b09a573140cb09fd7743d1624bd74d93875d8d0450c89214f2677
translation_provenance: en-US-german-reviewed
api_id: "ScanValue::Float32"
---
# ScanValue::Float32

Die SQL-Eingabevariante Float32 wählen.

```rust
Float32 (f32)
```

## Parameter, Rückgabe und übliche Verwendung

binary32 transportieren. Der Browser akzeptiert number, Rust verengt vor scan auf f32. NaN, Unendlichkeiten und Verengungsüberlauf liefern Unmarshal. Varianten werden von scan ausgeliehen und können innerhalb eines Browserlaufs wiederverwendet werden; erfundene Werte oder solche aus fremden Läufen werden abgewiesen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-float32" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
