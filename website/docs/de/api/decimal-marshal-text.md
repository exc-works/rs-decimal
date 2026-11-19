---
locale: de
source_revision: dabe3e417d155ec62acc296ebb36a3f87bdc044baf1a1ffd4eb27cbaf056b77d
translation_provenance: en-US-german-reviewed
api_id: "Decimal::marshal_text"
---
# Decimal::marshal_text

Decimal als text-Bytes codieren.

```rust
fn marshal_text (& self) -> Vec < u8 >
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert `Vec<u8>`/Uint8Array mit kanonischem ASCII-Dezimaltext. Nachgestellte Nachkommanullen werden entfernt. Die Standardnull von Decimal gibt ASCII 0 aus. Die zurückgegebenen Bytes sind eine unabhängige Kopie; Lesen ändert keinen Zustand.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-marshal-text" locale="de" />

## Verwandte Methoden

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
