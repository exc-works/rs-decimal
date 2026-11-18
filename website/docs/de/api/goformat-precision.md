---
locale: de
source_revision: b5c0056c42f3f4e6d3613af84d12321baa6359b54407d77f1aacdb7d5d883dd7
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

## Grenzen im Browser

Die [Ressourcenrichtlinie](../resource-policy) begrenzt Formatbreite auf 65536 Bytes und Präzision auf 1000; Festkommaformatierung prüft auch Koeffizientenwachstum. Puffer sind auf 65536 Bytes begrenzt. Append verwendet eine konservative Größenschätzung, die eine gerade noch passende Operation ablehnen kann. Ressourcenfehler verändern weder Eingaben noch Puffer. print kürzt lange Anzeigetexte separat; prüfen Sie für das vollständige Ergebnis die Länge oder geben Sie kleinere Abschnitte aus.

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
