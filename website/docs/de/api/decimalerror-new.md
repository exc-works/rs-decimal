---
locale: de
source_revision: 52cfb6e6f9d640e445d0e6b774455d4a9cdc44bf1d1bc94d3efd717749548fc4
translation_provenance: en-US-german-reviewed
api_id: "DecimalError::new"
---
# DecimalError::new

Einen Fehler mit einer Kategorie erzeugen.

```rust
fn new (kind : ErrorKind , message : impl Into < String >) -> Self
```

## Parameter, Rückgabe und übliche Verwendung

kind: ErrorKind, message: impl `Into<String>`/string; liefert einen neuen DecimalError. Die Meldung ist für Menschen bestimmt und legt Kategoriezugehörigkeit nicht fest. Der benannte Browserkonstruktor erzeugt einen freigebbaren Rust-basierten Wert mit message, matches(), kinds() und with_kind(). Geworfene Bibliotheksausnahmen sind native JS-Error-Objekte mit category="library" und kinds-Array; verwenden Sie dort e.kinds.includes(ErrorKind.Unmarshal). JavaScript-Fehler für parameter/resource/moved sind separat und besitzen keine Bibliotheksarten.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-new" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
