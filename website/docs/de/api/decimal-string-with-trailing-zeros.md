---
locale: de
source_revision: f320c1a30df05777794b5d83edd6494fed5ebfe3f6b80e1d48ebc83fa5e17748
translation_provenance: en-US-german-reviewed
api_id: Decimal::string_with_trailing_zeros
---
# Decimal::string_with_trailing_zeros

Dezimaltext in Festkommadarstellung formatieren.

```rust
fn string_with_trailing_zeros (& self) -> String
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert String/string ohne wissenschaftliche Notation. Erhält jede gespeicherte Nachkommastelle einschließlich nachgestellter Nullen. Verwendet einen Punkt und keine Gruppierung. Negative Null hat keine separate Darstellung. Die Standardnull wird als 0 ausgegeben, ohne initialisiert zu werden; Formatierung leiht die Eingabe aus.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-string-with-trailing-zeros" locale="de" />

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
