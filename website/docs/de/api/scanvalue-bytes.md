---
locale: de
source_revision: 69334c1aa4ddf9b63780cdd0f7b3c783c6e1646067bf4562ab026efd47279431
translation_provenance: en-US-german-reviewed
api_id: "ScanValue::Bytes"
---
# ScanValue::Bytes

Die SQL-Eingabevariante Bytes wählen.

```rust
Bytes (& 'a [u8])
```

## Parameter, Rückgabe und übliche Verwendung

Dezimaltextbytes ausleihen, keine binären/BSON-Übertragungsdaten. Der Browser hält das Uint8Array bis scan; vorherige Änderungen werden berücksichtigt. scan verändert die Eingabe nie. Ungültiges UTF-8 scheitert beim Dezimalparsen. Varianten werden von scan ausgeliehen und können innerhalb eines Browserlaufs wiederverwendet werden; erfundene Werte oder solche aus fremden Läufen werden abgewiesen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-bytes" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
