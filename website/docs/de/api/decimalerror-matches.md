---
locale: de
source_revision: 1c3bd02bc78216d73cebed78381413908e61e5b9b730c528a5271415d50a2935
translation_provenance: en-US-german-reviewed
api_id: "DecimalError::matches"
---
# DecimalError::matches

Zugehörigkeit zu einer Kategorie prüfen.

```rust
fn matches (& self , kind : ErrorKind) -> bool
```

## Parameter, Rückgabe und übliche Verwendung

kind: ErrorKind; liefert bool/boolean. Ausleihen verändert den Fehler nicht. Ein Fehler kann gleichzeitig mehreren Arten entsprechen. Der benannte Browserkonstruktor erzeugt einen freigebbaren Rust-basierten Wert mit message, matches(), kinds() und with_kind(). Geworfene Bibliotheksausnahmen sind native JS-Error-Objekte mit category="library" und kinds-Array; verwenden Sie dort e.kinds.includes(ErrorKind.Unmarshal). JavaScript-Fehler für parameter/resource/moved sind separat und besitzen keine Bibliotheksarten.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-matches" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
