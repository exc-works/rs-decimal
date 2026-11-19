---
locale: de
source_revision: 5f1761508e55494581ac3ef80177b3b3480f53a919c2efbe231b66c02ccf3a14
translation_provenance: en-US-german-reviewed
api_id: GoFormat.precision
---
# GoFormat.precision

Das Formatfeld precision setzen.

```rust
precision: Option < usize >
```

## Parameter, Rückgabe und übliche Verwendung

Die Bedeutung von precision hängt vom Formatierungszeichen ab: f/F verwenden Nachkommastellen, e/E Stellen nach der führenden Ziffer, g/G signifikante Ziffern (null wird als eins behandelt), d Mindestganzzahlziffern. None/null erhält die jeweiligen Standardwerte; f erhält die Skala, e verwendet sechs Stellen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-precision" locale="de" />

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
