---
locale: de
source_revision: 805fc62e495d042f613bdfd51817251dea41dcde63bd630737a6a7dd4ec14e8a
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

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
