---
locale: de
source_revision: c136bea39fff64039ef208c269a0655f2c126075df944740bc4d24f9d953ef55
translation_provenance: en-US-german-reviewed
api_id: "ErrorKind"
---
# ErrorKind

Stabile Bibliotheksfehlerkategorien beschreiben.

```rust
pub enum ErrorKind
```

## Parameter, Rückgabe und übliche Verwendung

Das Enum hat elf Varianten und kann in Rust kopiert und geordnet werden. Browser-ErrorKind ist eine aus Rust geladene eingefrorene Namensmap. Verwenden Sie Kategoriezugehörigkeit statt Meldungsvergleich für bestimmte Fehler. Der benannte Browserkonstruktor erzeugt einen freigebbaren Rust-basierten Wert mit message, matches(), kinds() und with_kind(). Geworfene Bibliotheksausnahmen sind native JS-Error-Objekte mit category="library" und kinds-Array; verwenden Sie dort e.kinds.includes(ErrorKind.Unmarshal). Bridge-Fehler für parameter/resource/moved sind separat und besitzen keine Bibliotheksarten.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind" locale="de" />

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
