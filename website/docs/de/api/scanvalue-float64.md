---
locale: de
source_revision: e5103a64d713be93c5c51f691b143dc290327bd825e0acd2029d936b2165437e
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

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
