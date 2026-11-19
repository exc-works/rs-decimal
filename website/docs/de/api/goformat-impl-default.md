---
locale: de
source_revision: 07a3f028e4aca5abdb58974bbb6c24c9b9018814af668c73f9dc49c66b12687d
translation_provenance: en-US-german-reviewed
api_id: GoFormat::impl Default
---
# GoFormat::impl Default

Die Rust-Implementierung von Default verwenden.

```rust
impl Default for GoFormat
```

## Parameter, Rückgabe und übliche Verwendung

GoFormat ist ein kleines Copy-Struct mit Feldern. Clone/Copy erhalten Optionen; Debug gibt Felder aus. Default wählt v, None für width/precision und false für alle Flags. Diese Rust-Syntax wird nativ getestet; JavaScript-Objekte verwenden gewöhnliche JS-Feldobjektsemantik und GoFormat.default().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-default" locale="de" />

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
