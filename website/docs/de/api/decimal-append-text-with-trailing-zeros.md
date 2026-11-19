---
locale: de
source_revision: 8f3a1f2c3a3bbab8fcd006696e4bbfc5801d795ba8c28546dfa164fc41913e69
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

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
