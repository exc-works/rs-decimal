---
locale: de
source_revision: 8805baa34ac15d1bc6208bedc88ae9d1d05c8235150f9f2331a9105561b652f3
translation_provenance: en-US-german-reviewed
api_id: "DecimalError"
---
# DecimalError

Eine Meldung und eine oder mehrere Bibliotheksfehlerarten behalten.

```rust
pub struct DecimalError
```

## Parameter, Rückgabe und übliche Verwendung

DecimalError speichert eine lesbare Meldung und geordnete, deduplizierte Kategorien. with_kind konsumiert den Wert; matches und kinds leihen ihn aus. Er implementiert Rust Error und Display. Der benannte Browserkonstruktor erzeugt einen freigebbaren Rust-basierten Wert mit message, matches(), kinds() und with_kind(). Geworfene Bibliotheksausnahmen sind native JS-Error-Objekte mit category="library" und kinds-Array; verwenden Sie dort e.kinds.includes(ErrorKind.Unmarshal). Bridge-Fehler für parameter/resource/moved sind separat und besitzen keine Bibliotheksarten.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
