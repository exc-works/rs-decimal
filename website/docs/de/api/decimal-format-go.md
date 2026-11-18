---
locale: de
source_revision: 0d692c90930959d30189d9870456cce3ed78a046b51b42c8004981e031486270
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

## Grenzen im Browser

Die [Ressourcenrichtlinie](../resource-policy) begrenzt Formatbreite auf 65536 Bytes und Präzision auf 1000; Festkommaformatierung prüft auch Koeffizientenwachstum. Puffer sind auf 65536 Bytes begrenzt. Append verwendet eine konservative Größenschätzung, die eine gerade noch passende Operation ablehnen kann. Ressourcenfehler verändern weder Eingaben noch Puffer. print kürzt lange Anzeigetexte separat; prüfen Sie für das vollständige Ergebnis die Länge oder geben Sie kleinere Abschnitte aus.

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
