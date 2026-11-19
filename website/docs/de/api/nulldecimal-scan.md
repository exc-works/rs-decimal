---
locale: de
source_revision: 3c6ebaee1d6c2d15a7801caee0a8597128eea9b110eb2748ac00d60cf69e23fd
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::scan"
---
# NullDecimal::scan

Einen treiberneutralen SQL-Skalar einlesen.

```rust
fn scan (& mut self , value : ScanValue < '_ >) -> Result < () , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

value: ScanValue; ändert den Empfänger und liefert `Result<(), DecimalError>`/undefined. Wählen Sie die Variante explizit: Int64 verwendet exaktes bigint, Float32 verengt auf binary32, Text/Bytes erhalten ursprüngliche Dezimalziffern. SQL-Text darf ein zusammengehöriges Paar äußerer doppelter Anführungszeichen haben; es wird ohne JSON-Escape-Auflösung entfernt. NULL löscht die Gültigkeit und setzt den inneren Decimal auf nicht initialisierte Null zurück; erfolgreiche Eingaben außer null setzen valid=true. Nicht unterstützte Typen und nicht endliche Gleitkommawerte liefern Unmarshal. Andere Decodierfehler behalten zugrunde liegende Arten. Alle Fehler erhalten den Empfängerzustand. Dieser Adapter benötigt keine Datenbankverbindung.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-nulldecimal-scan" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
