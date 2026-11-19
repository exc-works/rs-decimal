---
locale: de
source_revision: fe33f9fd8631eeeae5a499a47abbc262340fc190757785abe0749a14c97bf791
translation_provenance: en-US-german-reviewed
api_id: GoFormat.width
---
# GoFormat.width

Das Formatfeld width setzen.

```rust
width: Option < usize >
```

## Parameter, Rückgabe und übliche Verwendung

Die Mindestbreite umfasst Vorzeichen und Dezimalpunkt; sie füllt auf, ohne längeren Text abzuschneiden. None/null deaktiviert die Mindestbreite. Die Kernformatierung zählt Ausgabebytes.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-width" locale="de" />

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
