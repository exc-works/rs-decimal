---
locale: de
source_revision: 4a909248a2e14bd655a50dd72e826f6d87a3347b735e76ddfe31ae9a4d4b4671
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

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
