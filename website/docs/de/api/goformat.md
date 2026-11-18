---
locale: de
source_revision: e9b605c9c2b762d25f61c081bee35aa1c375be0cb3cab1e12e2bd0938c7a3c70
translation_provenance: en-US-german-reviewed
api_id: GoFormat
---
# GoFormat

Eine einzelne Formatierung mit einem Feldobjekt beschreiben.

```rust
pub struct GoFormat
```

## Parameter, Rückgabe und übliche Verwendung

Die Felder entsprechen dem Rust-Struct: verb, width, precision, left, plus, space, zero und alternate. GoFormat.default() in der Bridge liest Rust-Standardwerte und liefert ein neues veränderliches Objekt; dessen Änderung setzt keine globale Formatierung. width/precision verwenden null für None und nichtnegative wasm32-Ganzzahlen vom Typ number für Some. Rust-Traits bleiben rein native Syntax.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat" locale="de" />

## Grenzen im Browser

Die [Ressourcenrichtlinie](../resource-policy) begrenzt Formatbreite auf 65536 Bytes und Präzision auf 1000; Festkommaformatierung prüft auch Koeffizientenwachstum. Puffer sind auf 65536 Bytes begrenzt. Append verwendet eine konservative Größenschätzung, die eine gerade noch passende Operation ablehnen kann. Ressourcenfehler verändern weder Eingaben noch Puffer. print kürzt lange Anzeigetexte separat; prüfen Sie für das vollständige Ergebnis die Länge oder geben Sie kleinere Abschnitte aus.

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
