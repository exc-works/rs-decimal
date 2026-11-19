---
locale: de
source_revision: 873d615adf07acd5810e5a3d2447199c951d65cd40cb07c1afb47b23535aad17
translation_provenance: en-US-german-reviewed
api_id: GoFormat::impl Default::default
---
# GoFormat::impl Default::default

Die Rust-Implementierung von Default::default verwenden.

```rust
fn default () -> Self
```

## Parameter, Rückgabe und übliche Verwendung

GoFormat ist ein kleines Copy-Struct mit Feldern. Clone/Copy erhalten Optionen; Debug gibt Felder aus. Default wählt v, None für width/precision und false für alle Flags. Diese Rust-Syntax wird nativ getestet; Bridge-Objekte verwenden gewöhnliche JS-Feldobjektsemantik und GoFormat.default().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-default-default" locale="de" />

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
