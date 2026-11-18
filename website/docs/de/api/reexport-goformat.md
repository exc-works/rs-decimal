---
locale: de
source_revision: 4beaa8ccdf2751ba7bce74d134b8de29fabd5c0d82d0d4eebee1575a204be12d
translation_provenance: en-US-german-reviewed
api_id: reexport::GoFormat
---
# reexport::GoFormat

GoFormat aus der Crate-Wurzel importieren.

```rust
pub use formatting::GoFormat as GoFormat
```

## Parameter, Rückgabe und übliche Verwendung

Dies ist der öffentliche Reexport des Rust-Structs, keine weitere Implementierung. Der Browser bietet einen Feldobjektadapter und eine auf Rust basierende Standardwertfabrik.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-reexport-goformat" locale="de" />

## Grenzen im Browser

Die [Ressourcenrichtlinie](../resource-policy) begrenzt Formatbreite auf 65536 Bytes und Präzision auf 1000; Festkommaformatierung prüft auch Koeffizientenwachstum. Puffer sind auf 65536 Bytes begrenzt. Append verwendet eine konservative Größenschätzung, die eine gerade noch passende Operation ablehnen kann. Ressourcenfehler verändern weder Eingaben noch Puffer. print kürzt lange Anzeigetexte separat; prüfen Sie für das vollständige Ergebnis die Länge oder geben Sie kleinere Abschnitte aus.

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
