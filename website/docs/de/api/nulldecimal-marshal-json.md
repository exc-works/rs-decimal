---
locale: de
source_revision: f90f9a719348f32512d576fc2ef23076c6492acd11b0dbe87ae7f8c72a7c2eab
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::marshal_json"
---
# NullDecimal::marshal_json

NullDecimal als json-Bytes codieren.

```rust
fn marshal_json (& self) -> Vec < u8 >
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert `Vec<u8>`/Uint8Array mit JSON-Bytes. Rust benötigt json; der Browserbuild aktiviert es. Ungültige nullable-Werte werden als JSON null codiert; gültige Werte als kanonische Dezimalzeichenfolgen in Anführungszeichen, auch bei nicht initialisiertem innerem Decimal. Die zurückgegebenen Bytes sind eine unabhängige Kopie; Lesen ändert keinen Zustand.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-marshal-json" locale="de" />

## Verwandte Methoden

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
