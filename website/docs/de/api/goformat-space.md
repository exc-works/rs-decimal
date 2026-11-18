---
locale: de
source_revision: 6ee970e3e2507cdd225eb4c314175b5a95b9aa45ac2e64ed4be5fded0c11b425
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

## Grenzen im Browser

Die [Ressourcenrichtlinie](../resource-policy) begrenzt Formatbreite auf 65536 Bytes und Präzision auf 1000; Festkommaformatierung prüft auch Koeffizientenwachstum. Puffer sind auf 65536 Bytes begrenzt. Append verwendet eine konservative Größenschätzung, die eine gerade noch passende Operation ablehnen kann. Ressourcenfehler verändern weder Eingaben noch Puffer. print kürzt lange Anzeigetexte separat; prüfen Sie für das vollständige Ergebnis die Länge oder geben Sie kleinere Abschnitte aus.

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
