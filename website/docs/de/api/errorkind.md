---
locale: de
source_revision: 3bfbb5926961d9d892e3da2738bc0fc6133b23f056d8610803f8a02d24a533d4
translation_provenance: en-US-german-reviewed
api_id: "ErrorKind"
---
# ErrorKind

Stabile Bibliotheksfehlerkategorien beschreiben.

```rust
pub enum ErrorKind
```

## Parameter, Rückgabe und übliche Verwendung

Das Enum hat elf Varianten und kann in Rust kopiert und geordnet werden. Browser-ErrorKind ist eine aus Rust geladene eingefrorene Namensmap. Verwenden Sie Kategoriezugehörigkeit statt Meldungsvergleich für bestimmte Fehler. Der benannte Browserkonstruktor erzeugt einen freigebbaren Rust-basierten Wert mit message, matches(), kinds() und with_kind(). Geworfene Bibliotheksausnahmen sind native JS-Error-Objekte mit category="library" und kinds-Array; verwenden Sie dort e.kinds.includes(ErrorKind.Unmarshal). Bridge-Fehler für parameter/resource/moved sind separat und besitzen keine Bibliotheksarten.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind" locale="de" />

## Verwandte Methoden

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
