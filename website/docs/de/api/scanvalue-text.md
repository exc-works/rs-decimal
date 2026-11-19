---
locale: de
source_revision: 66f48b07b8a5c080eb29b01047825ba3448ab4e979a526ae96e347e482fef177
translation_provenance: en-US-german-reviewed
api_id: "ScanValue::Text"
---
# ScanValue::Text

Die SQL-Eingabevariante Text wählen.

```rust
Text (& 'a str)
```

## Parameter, Rückgabe und übliche Verwendung

Exakten Dezimaltext ausleihen. Ein zusammengehöriges Paar äußerer doppelter Anführungszeichen wird entfernt; Escape-Sequenzen werden nicht als JSON decodiert. Nachgestellte Skala bei Zahlenwerten ungleich null bleibt nach dem Parsen erhalten. Varianten werden von scan ausgeliehen und können innerhalb eines Browserlaufs wiederverwendet werden; erfundene Werte oder solche aus fremden Läufen werden abgewiesen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-text" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
