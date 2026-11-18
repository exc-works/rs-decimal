---
locale: de
source_revision: b5a3d9a8d6370fd602bde442c0194ed8033b601f5d907d38c4cf9f51317197c6
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

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
