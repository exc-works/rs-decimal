---
locale: de
source_revision: 0c3a3cc5d3969cdd2cb971de3686daa945e18486f55f8c99179deb863727c501
translation_provenance: en-US-german-reviewed
api_id: GoFormat.verb
---
# GoFormat.verb

Das Formatfeld verb setzen.

```rust
verb: char
```

## Parameter, Rückgabe und übliche Verwendung

Einen Unicode-Skalarwert als Formatierungszeichen wählen. Standard v verwendet kompakten Festkommatext; e/E wissenschaftliche Notation, q Anführungszeichen, b den binären Koeffizienten mit anschließendem p-scale (keinen IEEE-Gleitkommawert).

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-verb" locale="de" />

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
