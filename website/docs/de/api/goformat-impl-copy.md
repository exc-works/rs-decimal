---
locale: de
source_revision: d5ce77b83bcc1f28279c44d6d96fc602e4eb59aa65aad6eb814e4e715f060309
translation_provenance: en-US-german-reviewed
api_id: GoFormat::impl Copy
---
# GoFormat::impl Copy

Die Rust-Implementierung von Copy verwenden.

```rust
impl Copy for GoFormat
```

## Parameter, Rückgabe und übliche Verwendung

GoFormat ist ein kleines Copy-Struct mit Feldern. Clone/Copy erhalten Optionen; Debug gibt Felder aus. Default wählt v, None für width/precision und false für alle Flags. Diese Rust-Syntax wird nativ getestet; Bridge-Objekte verwenden gewöhnliche JS-Feldobjektsemantik und GoFormat.default().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-copy" locale="de" />

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
