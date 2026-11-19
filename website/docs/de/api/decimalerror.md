---
locale: de
source_revision: 51df12e136a0b4ed98ea9f557c4e61c7872827fb92cbe6a97475d66177b43284
translation_provenance: en-US-german-reviewed
api_id: "DecimalError"
---
# DecimalError

Eine Meldung und eine oder mehrere Bibliotheksfehlerarten behalten.

```rust
pub struct DecimalError
```

## Parameter, Rückgabe und übliche Verwendung

DecimalError speichert eine lesbare Meldung und geordnete, deduplizierte Kategorien. with_kind konsumiert den Wert; matches und kinds leihen ihn aus. Er implementiert Rust Error und Display. Der benannte Browserkonstruktor erzeugt einen freigebbaren Rust-basierten Wert mit message, matches(), kinds() und with_kind(). Geworfene Bibliotheksausnahmen sind native JS-Error-Objekte mit category="library" und kinds-Array; verwenden Sie dort e.kinds.includes(ErrorKind.Unmarshal). JavaScript-Fehler für parameter/resource/moved sind separat und besitzen keine Bibliotheksarten.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
