---
locale: de
source_revision: 9028fbd5554cd396883d5b8bb4b06efa13e22d1b025f5ba855a44ab02e548531
translation_provenance: en-US-german-reviewed
api_id: Decimal::append_text_with_trailing_zeros
---
# Decimal::append_text_with_trailing_zeros

Dezimale ASCII-Bytes an einen vorhandenen veränderlichen Puffer anhängen.

```rust
fn append_text_with_trailing_zeros (& self , dst : & mut Vec < u8 >)
```

## Parameter, Rückgabe und übliche Verwendung

`dst: &mut Vec<u8>`, dargestellt durch einen ByteBuffer-Wrapper. Rust liefert (), JS undefined. Vorhandene Bytes bleiben erhalten; der Zielpuffer wird direkt verändert. Alle gespeicherten Nachkommanullen bleiben erhalten. Decimal und Puffer bleiben nutzbar. Ein Uint8Array wird in ByteBuffer.new kopiert; to_uint8_array liefert eine Kopie, keine Wasm-Speicheransicht. Ressourcenfehler treten vor der Änderung auf und lassen den Puffer unverändert.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-append-text-with-trailing-zeros" locale="de" />

## Grenzen im Browser

Die [Ressourcenrichtlinie](../resource-policy) begrenzt Formatbreite auf 65536 Bytes und Präzision auf 1000; Festkommaformatierung prüft auch Koeffizientenwachstum. Puffer sind auf 65536 Bytes begrenzt. Append verwendet eine konservative Größenschätzung, die eine gerade noch passende Operation ablehnen kann. Ressourcenfehler verändern weder Eingaben noch Puffer. print kürzt lange Anzeigetexte separat; prüfen Sie für das vollständige Ergebnis die Länge oder geben Sie kleinere Abschnitte aus.

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
