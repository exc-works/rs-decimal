---
locale: de
source_revision: c5619d120e2ab677bb25ae58cef9205a1160d82cd87b1081f2a6532750a83303
translation_provenance: en-US-german-reviewed
api_id: GoFormat.space
---
# GoFormat.space

Das Formatfeld space setzen.

```rust
space: bool
```

## Parameter, Rückgabe und übliche Verwendung

Bei nichtnegativer numerischer Ausgabe ein führendes Leerzeichen ergänzen, wenn plus false ist. Negative Werte behalten ihr Vorzeichen. Standard: false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-space" locale="de" />

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
