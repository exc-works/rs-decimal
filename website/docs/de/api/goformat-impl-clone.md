---
locale: de
source_revision: df43bbe7e732dc8914c18a95d0b5d68a3839eb4a2b1c83d4dc811e0031827a91
translation_provenance: en-US-german-reviewed
api_id: GoFormat::impl Clone
---
# GoFormat::impl Clone

Die Rust-Implementierung von Clone verwenden.

```rust
impl Clone for GoFormat
```

## Parameter, Rückgabe und übliche Verwendung

GoFormat ist ein kleines Copy-Struct mit Feldern. Clone/Copy erhalten Optionen; Debug gibt Felder aus. Default wählt v, None für width/precision und false für alle Flags. Diese Rust-Syntax wird nativ getestet; Bridge-Objekte verwenden gewöhnliche JS-Feldobjektsemantik und GoFormat.default().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-clone" locale="de" />

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
