---
locale: de
source_revision: 0989f60fe84cf7be4e7708a7140f9cc16468c21ad57d855b67f896084ddd8606
translation_provenance: en-US-german-reviewed
api_id: GoFormat.plus
---
# GoFormat.plus

Das Formatfeld plus setzen.

```rust
plus: bool
```

## Parameter, Rückgabe und übliche Verwendung

Bei nichtnegativer numerischer Ausgabe + anzeigen. Hat Vorrang vor space. Gilt für numerische Formatierungszeichen, nicht v/s/q; negative Werte behalten -. Standard: false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-plus" locale="de" />

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
