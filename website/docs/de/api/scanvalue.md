---
locale: de
source_revision: bc03546de8ea5c2e53e57f5ce62fd19b75d7cd5d6737f4aa1452dbcef13f206d
translation_provenance: en-US-german-reviewed
api_id: "ScanValue"
---
# ScanValue

Eine treiberneutrale typisierte SQL-Eingabe wählen.

```rust
pub enum ScanValue< 'a >
```

## Parameter, Rückgabe und übliche Verwendung

Das Enum unterscheidet NULL, f32, f64, i64, Text, Textbytes und nicht unterstützte Werte. Wählen Sie vor Decimal.scan oder NullDecimal.scan eine explizite Variante. Ausgeliehene Texte/Bytes müssen in Rust nur während des Aufrufs gültig sein; Browser-Bytes hält das Array des Aufrufers und berücksichtigt Änderungen vor scan. Es wird kein Datenbankdienst kontaktiert.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue" locale="de" />

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
