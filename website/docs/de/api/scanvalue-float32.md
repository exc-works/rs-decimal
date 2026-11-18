---
locale: de
source_revision: dbb045138dac94f7b2691e8576e6af500d5ead689869256e178b5069283ab80d
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

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
