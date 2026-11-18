---
locale: de
source_revision: 349cbd7b8c032b6b9b799b90d2d1981de5ba6641cd94e6d5d2f6cdc9c436f5e2
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

## Grenzen im Browser

Die [Ressourcenrichtlinie](../resource-policy) begrenzt Formatbreite auf 65536 Bytes und Präzision auf 1000; Festkommaformatierung prüft auch Koeffizientenwachstum. Puffer sind auf 65536 Bytes begrenzt. Append verwendet eine konservative Größenschätzung, die eine gerade noch passende Operation ablehnen kann. Ressourcenfehler verändern weder Eingaben noch Puffer. print kürzt lange Anzeigetexte separat; prüfen Sie für das vollständige Ergebnis die Länge oder geben Sie kleinere Abschnitte aus.

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
