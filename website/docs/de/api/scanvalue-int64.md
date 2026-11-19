---
locale: de
source_revision: 1534eba12b3e65df06a4bea76e8dd9c9cd12eeb6bcfbce92f69f83b923eef4c3
translation_provenance: en-US-german-reviewed
api_id: "ScanValue::Int64"
---
# ScanValue::Int64

Die SQL-Eingabevariante Int64 wählen.

```rust
Int64 (i64)
```

## Parameter, Rückgabe und übliche Verwendung

i64 exakt transportieren. Der Browser verlangt bigint im vorzeichenbehafteten 64-Bit-Bereich; JS number wird abgewiesen. Varianten werden von scan ausgeliehen und können innerhalb eines Browserlaufs wiederverwendet werden; erfundene Werte oder solche aus fremden Läufen werden abgewiesen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-int64" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
