---
locale: de
source_revision: f89bca6a9b41c5dfaa6d534efc134c1f42c007ba065d09915785753b5a161115
translation_provenance: en-US-german-reviewed
api_id: GoFormat
---
# GoFormat

Eine einzelne Formatierung mit einem Feldobjekt beschreiben.

```rust
pub struct GoFormat
```

## Parameter, Rückgabe und übliche Verwendung

Die Felder entsprechen dem Rust-Struct: verb, width, precision, left, plus, space, zero und alternate. GoFormat.default() in der Bridge liest Rust-Standardwerte und liefert ein neues veränderliches Objekt; dessen Änderung setzt keine globale Formatierung. width/precision verwenden null für None und nichtnegative wasm32-Ganzzahlen vom Typ number für Some. Rust-Traits bleiben rein native Syntax.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat" locale="de" />

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
