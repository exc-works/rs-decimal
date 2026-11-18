---
locale: de
source_revision: 292844a00b6e84cbd25e16d6e84fa03c5565c434cf287f28e2959e5ca0c46780
translation_provenance: en-US-german-reviewed
api_id: "DecimalError::matches"
---
# DecimalError::matches

Zugehörigkeit zu einer Kategorie prüfen.

```rust
fn matches (& self , kind : ErrorKind) -> bool
```

## Parameter, Rückgabe und übliche Verwendung

kind: ErrorKind; liefert bool/boolean. Ausleihen verändert den Fehler nicht. Ein Fehler kann gleichzeitig mehreren Arten entsprechen. Der benannte Browserkonstruktor erzeugt einen freigebbaren Rust-basierten Wert mit message, matches(), kinds() und with_kind(). Geworfene Bibliotheksausnahmen sind native JS-Error-Objekte mit category="library" und kinds-Array; verwenden Sie dort e.kinds.includes(ErrorKind.Unmarshal). Bridge-Fehler für parameter/resource/moved sind separat und besitzen keine Bibliotheksarten.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-matches" locale="de" />

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
