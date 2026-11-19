---
locale: de
source_revision: b5003d3283929da01970193aca2fe128bcc80043185972b8b1e270211629771c
translation_provenance: en-US-german-reviewed
api_id: "Decimal::scan"
---
# Decimal::scan

Einen treiberneutralen SQL-Skalar einlesen.

```rust
fn scan (& mut self , value : ScanValue < '_ >) -> Result < () , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

value: ScanValue; ändert den Empfänger und liefert `Result<(), DecimalError>`/undefined. Wählen Sie die Variante explizit: Int64 verwendet exaktes bigint, Float32 verengt auf binary32, Text/Bytes erhalten ursprüngliche Dezimalziffern. SQL-Text darf ein zusammengehöriges Paar äußerer doppelter Anführungszeichen haben; es wird ohne JSON-Escape-Auflösung entfernt. NULL setzt den Empfänger auf nicht initialisierte Null zurück. Nicht unterstützte Typen und nicht endliche Gleitkommawerte liefern Unmarshal. Andere Decodierfehler behalten zugrunde liegende Arten. Alle Fehler erhalten den Empfängerzustand. Dieser Adapter benötigt keine Datenbankverbindung.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimal-scan" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
