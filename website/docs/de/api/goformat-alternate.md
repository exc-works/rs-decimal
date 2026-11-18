---
locale: de
source_revision: 5323ce455eba922d5a5c86c41373b1d3638bbe014511b21ca1e6f036392adab6
translation_provenance: en-US-german-reviewed
api_id: GoFormat.alternate
---
# GoFormat.alternate

Das Formatfeld alternate setzen.

```rust
alternate: bool
```

## Parameter, Rückgabe und übliche Verwendung

Für f/F/e/E einen Dezimalpunkt beibehalten und für g/G nachgestellte Nullen im allgemeinen Format erhalten. Dies bedeutet keine alternative Zahlenbasis. Standard: false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-alternate" locale="de" />

## Grenzen im Browser

Die [Ressourcenrichtlinie](../resource-policy) begrenzt Formatbreite auf 65536 Bytes und Präzision auf 1000; Festkommaformatierung prüft auch Koeffizientenwachstum. Puffer sind auf 65536 Bytes begrenzt. Append verwendet eine konservative Größenschätzung, die eine gerade noch passende Operation ablehnen kann. Ressourcenfehler verändern weder Eingaben noch Puffer. print kürzt lange Anzeigetexte separat; prüfen Sie für das vollständige Ergebnis die Länge oder geben Sie kleinere Abschnitte aus.

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
