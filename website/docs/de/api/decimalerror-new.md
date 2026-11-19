---
locale: de
source_revision: 634fa480d38c38e977bcb656a0df8ea6f905528b83e0983585b20f7e954e4020
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

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
