---
locale: de
source_revision: 51be5745f7c3de3ef96d99823418ed54f498b3a9d7e9e49834c1ccd963cd3475
translation_provenance: en-US-german-reviewed
api_id: "DecimalError::kinds"
---
# DecimalError::kinds

Alle Fehlerkategorien lesen.

```rust
fn kinds (& self) -> & [ErrorKind]
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert &[ErrorKind]/ein unabhängiges Array von Variantennamen. Kategorien werden dedupliziert und nach Rust-Enumreihenfolge statt alphabetisch sortiert. Der benannte Browserkonstruktor erzeugt einen freigebbaren Rust-basierten Wert mit message, matches(), kinds() und with_kind(). Geworfene Bibliotheksausnahmen sind native JS-Error-Objekte mit category="library" und kinds-Array; verwenden Sie dort e.kinds.includes(ErrorKind.Unmarshal). Bridge-Fehler für parameter/resource/moved sind separat und besitzen keine Bibliotheksarten.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-kinds" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
