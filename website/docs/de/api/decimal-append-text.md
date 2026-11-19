---
locale: de
source_revision: dd4986a8dd71923263f954c74b5685b1dfaa3a4b594ef1aadb36c8a62519bd62
translation_provenance: en-US-german-reviewed
api_id: Decimal::append_text
---
# Decimal::append_text

Dezimale ASCII-Bytes an einen vorhandenen veränderlichen Puffer anhängen.

```rust
fn append_text (& self , dst : & mut Vec < u8 >)
```

## Parameter, Rückgabe und übliche Verwendung

`dst: &mut Vec<u8>`, dargestellt durch einen ByteBuffer-Wrapper. Rust liefert (), JS undefined. Vorhandene Bytes bleiben erhalten; der Zielpuffer wird direkt verändert. Nachgestellte Nachkommanullen werden entfernt. Decimal und Puffer bleiben nutzbar. Ein Uint8Array wird in ByteBuffer.new kopiert; to_uint8_array liefert eine Kopie, keine Wasm-Speicheransicht. Ressourcenfehler treten vor der Änderung auf und lassen den Puffer unverändert.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-append-text" locale="de" />

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
