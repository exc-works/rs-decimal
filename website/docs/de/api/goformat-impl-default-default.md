---
locale: de
source_revision: 6a03ad3f2e6f0755e57eb89608d0891566bd0e9c4b875afd8935ad4e0e27dd6d
translation_provenance: en-US-german-reviewed
api_id: GoFormat::impl Default::default
---
# GoFormat::impl Default::default

Die Rust-Implementierung von Default::default verwenden.

```rust
fn default () -> Self
```

## Parameter, Rückgabe und übliche Verwendung

GoFormat ist ein kleines Copy-Struct mit Feldern. Clone/Copy erhalten Optionen; Debug gibt Felder aus. Default wählt v, None für width/precision und false für alle Flags. Diese Rust-Syntax wird nativ getestet; JavaScript-Objekte verwenden gewöhnliche JS-Feldobjektsemantik und GoFormat.default().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-default-default" locale="de" />

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
