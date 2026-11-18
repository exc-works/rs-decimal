---
locale: de
source_revision: 84f61035965dcc9b605c00e7c877afbafcb7870fb2b3b89e0902348c80e03dd7
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

## Grenzen im Browser

Die [Ressourcenrichtlinie](../resource-policy) begrenzt Formatbreite auf 65536 Bytes und Präzision auf 1000; Festkommaformatierung prüft auch Koeffizientenwachstum. Puffer sind auf 65536 Bytes begrenzt. Append verwendet eine konservative Größenschätzung, die eine gerade noch passende Operation ablehnen kann. Ressourcenfehler verändern weder Eingaben noch Puffer. print kürzt lange Anzeigetexte separat; prüfen Sie für das vollständige Ergebnis die Länge oder geben Sie kleinere Abschnitte aus.

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
