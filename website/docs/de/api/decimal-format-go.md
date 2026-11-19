---
locale: de
source_revision: f9b1eaa0c6a10e9f32908baa1f2c09a9aafeb3265bffa095bb0a0b210e1e1f05
translation_provenance: en-US-german-reviewed
api_id: Decimal::format_go
---
# Decimal::format_go

Ein Go-kompatibles Formatierungszeichen mit Formatierungsoptionen anwenden.

```rust
fn format_go (& self , f : GoFormat) -> Result < String , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

f: GoFormat; liefert Result<String, DecimalError>/string. Akzeptiert ein einzelnes Formatierungszeichen, kein printf-Muster wie %.2f. Unterstützt werden v/s/q/d/f/F/e/E/g/G/b. Gerundet wird HalfEven. Nicht unterstützte Zeichen und nichtganzzahliges d erzeugen eine %!-Markierung als Text, keine Ausnahme. Festkommapräzision außerhalb des nativen vorzeichenbehafteten Bereichs liefert InvalidPrecision; Browsergrenzen für Breite/Präzision/Allokation sind separat. Fehlende JS-Felder verwenden Rust-Standardwerte; unbekannte Felder und ungültige Feldtypen werden abgewiesen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-format-go" locale="de" />

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
