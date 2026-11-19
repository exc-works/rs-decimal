---
locale: de
source_revision: c3fbe0927c540d91118929ccf7291b490ab739216719c7655f9f81744f8edff3
translation_provenance: en-US-german-reviewed
api_id: Decimal::format_with_separators
---
# Decimal::format_with_separators

Ganzzahlziffern gruppieren und das Dezimaltrennzeichen wählen.

```rust
fn format_with_separators (& self , thousands : char , decimal : char) -> String
```

## Parameter, Rückgabe und übliche Verwendung

thousands: char; decimal: char. Liefert String/string mit Gruppen aus drei Ganzzahlziffern und sämtlichen gespeicherten Nachkommastellen. Jedes JS-Argument muss genau ein Unicode-Skalarwert sein, einschließlich Zeichen außerhalb der BMP; einzelne Surrogate und Zeichenfolgen mit mehreren Zeichen werden abgewiesen. NUL für thousands deaktiviert Gruppierung; NUL für decimal wählt einen Punkt. Trennzeichen sind wörtliche Zeichen, keine Gebietsschemanamen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-format-with-separators" locale="de" />

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
