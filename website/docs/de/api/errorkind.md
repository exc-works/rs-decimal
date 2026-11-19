---
locale: de
source_revision: e6b8dfbe3d3140a4697ec5a94f4d508f975bc9c0847e91c70ef29e95978ddcfb
translation_provenance: en-US-german-reviewed
api_id: "ErrorKind"
---
# ErrorKind

Stabile Bibliotheksfehlerkategorien beschreiben.

```rust
pub enum ErrorKind
```

## Parameter, Rückgabe und übliche Verwendung

Das Enum hat elf Varianten und kann in Rust kopiert und geordnet werden. Browser-ErrorKind ist eine aus Rust geladene eingefrorene Namensmap. Verwenden Sie Kategoriezugehörigkeit statt Meldungsvergleich für bestimmte Fehler. Der benannte Browserkonstruktor erzeugt einen freigebbaren Rust-basierten Wert mit message, matches(), kinds() und with_kind(). Geworfene Bibliotheksausnahmen sind native JS-Error-Objekte mit category="library" und kinds-Array; verwenden Sie dort e.kinds.includes(ErrorKind.Unmarshal). JavaScript-Fehler für parameter/resource/moved sind separat und besitzen keine Bibliotheksarten.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
