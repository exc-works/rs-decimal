---
locale: de
source_revision: c70b9c986ae1c2d1db6f94784745de5a4d3bb8a7cf3f6af3550e9be48e0f551f
translation_provenance: en-US-german-reviewed
api_id: "DecimalError::new"
---
# DecimalError::new

Einen Fehler mit einer Kategorie erzeugen.

```rust
fn new (kind : ErrorKind , message : impl Into < String >) -> Self
```

## Parameter, Rückgabe und übliche Verwendung

kind: ErrorKind, message: impl `Into<String>`/string; liefert einen neuen DecimalError. Die Meldung ist für Menschen bestimmt und legt Kategoriezugehörigkeit nicht fest. Der benannte Browserkonstruktor erzeugt einen freigebbaren Rust-basierten Wert mit message, matches(), kinds() und with_kind(). Geworfene Bibliotheksausnahmen sind native JS-Error-Objekte mit category="library" und kinds-Array; verwenden Sie dort e.kinds.includes(ErrorKind.Unmarshal). Bridge-Fehler für parameter/resource/moved sind separat und besitzen keine Bibliotheksarten.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-new" locale="de" />

## Grenzen im Browser

SQL-Text-/Byteeingaben und Fehlermeldungen sind auf 8192 UTF-8-Bytes begrenzt. SQL-Text wird nach Entfernen äußerer Anführungszeichen [auf Ressourcen geprüft](../resource-policy), sodass Zitieren keine Exponentengrenzen umgeht. Ausleihende Aufrufe erhalten Eingaben; konsumierte oder freigegebene Handles sind nicht wiederverwendbar. Rust-Trait-/Importbeispiele laufen in nativen Tests und behaupten keine Browsermethodenunterstützung.

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
