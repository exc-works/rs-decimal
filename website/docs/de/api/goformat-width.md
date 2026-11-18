---
locale: de
source_revision: 0ad883afdae2ec6d62af55006e383eb095f5c7928dcdbf6c2f7baf446a49d945
translation_provenance: en-US-german-reviewed
api_id: GoFormat.width
---
# GoFormat.width

Das Formatfeld width setzen.

```rust
width: Option < usize >
```

## Parameter, Rückgabe und übliche Verwendung

Die Mindestbreite umfasst Vorzeichen und Dezimalpunkt; sie füllt auf, ohne längeren Text abzuschneiden. None/null deaktiviert die Mindestbreite. Die Kernformatierung zählt Ausgabebytes.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-width" locale="de" />

## Grenzen im Browser

Die [Ressourcenrichtlinie](../resource-policy) begrenzt Formatbreite auf 65536 Bytes und Präzision auf 1000; Festkommaformatierung prüft auch Koeffizientenwachstum. Puffer sind auf 65536 Bytes begrenzt. Append verwendet eine konservative Größenschätzung, die eine gerade noch passende Operation ablehnen kann. Ressourcenfehler verändern weder Eingaben noch Puffer. print kürzt lange Anzeigetexte separat; prüfen Sie für das vollständige Ergebnis die Länge oder geben Sie kleinere Abschnitte aus.

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
