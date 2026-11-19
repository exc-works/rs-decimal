---
locale: de
source_revision: 51d8eb06be2cf1b2c76ae713dc34443717bd6adc00c52dec95e805914eed6ec1
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

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
