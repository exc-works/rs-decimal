---
locale: de
source_revision: 2b341a7995c77dbb72c2087390bcd56b4208d2c9ec45de8ea22c2ce3f209501a
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::marshal_text"
---
# NullDecimal::marshal_text

NullDecimal als text-Bytes codieren.

```rust
fn marshal_text (& self) -> Vec < u8 >
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert `Vec<u8>`/Uint8Array mit kanonischem ASCII-Dezimaltext. Gültige Werte codieren kanonischen Dezimaltext; ungültige Werte leere Bytes, nicht das Wort null. Die zurückgegebenen Bytes sind eine unabhängige Kopie; Lesen ändert keinen Zustand.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-marshal-text" locale="de" />

## Verwandte Methoden

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
