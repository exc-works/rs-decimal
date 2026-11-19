---
locale: de
source_revision: 25693aaef850a3e88dcc638a63de0aa32a65fe1057c9bac969c2400862072704
translation_provenance: en-US-german-reviewed
api_id: Decimal::string
---
# Decimal::string

Dezimaltext in Festkommadarstellung formatieren.

```rust
fn string (& self) -> String
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert String/string ohne wissenschaftliche Notation. Entfernt überflüssige nachgestellte Nachkommanullen; ganzzahlige Nullen bleiben. Verwendet einen Punkt und keine Gruppierung. Negative Null hat keine separate Darstellung. Die Standardnull wird als 0 ausgegeben, ohne initialisiert zu werden; Formatierung leiht die Eingabe aus.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-string" locale="de" />

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
