---
locale: de
source_revision: 76052d80c386b89ac2428898d57b9ce525f22161fd5cd3ee7956cceafe3b11e9
translation_provenance: en-US-german-reviewed
api_id: "Decimal::marshal_json"
---
# Decimal::marshal_json

Decimal als json-Bytes codieren.

```rust
fn marshal_json (& self) -> Vec < u8 >
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert `Vec<u8>`/Uint8Array mit JSON-Bytes. Rust benötigt json; der Browserbuild aktiviert es. Initialisierte Decimal-Werte werden als kanonische Zeichenfolgen in Anführungszeichen codiert; nicht initialisierte Standardnull als JSON null. Die zurückgegebenen Bytes sind eine unabhängige Kopie; Lesen ändert keinen Zustand.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-marshal-json" locale="de" />

## Verwandte Methoden

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
