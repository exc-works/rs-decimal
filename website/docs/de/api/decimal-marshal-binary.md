---
locale: de
source_revision: aa67b8e6e6f3fb7b6a66f11573d9907fb9dc057abd9547e4d0c0474969528084
translation_provenance: en-US-german-reviewed
api_id: "Decimal::marshal_binary"
---
# Decimal::marshal_binary

Das Go-kompatible precision/Gob-Übertragungsformat codieren.

```rust
fn marshal_binary (& self) -> Result < Vec < u8 > , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert `Result<Vec<u8>, DecimalError>`/Uint8Array. Initialisierte Werte entfernen nachgestellte Nachkommanullen, schreiben dann eine vier Byte lange Big-Endian-Skala, das big.Int-Gob-Versions-/Vorzeichenbyte und den Big-Endian-Absolutwert. Dies ist eine dezimale Skalar-Payload, kein IEEE-Gleitkommawert. Nicht initialisierte Null ergibt leere Bytes; initialisierte Null ergibt [0,0,0,0,2]. Die zurückgegebene Kopie kann ohne Änderung des Decimal verändert werden.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-marshal-binary" locale="de" />

## Verwandte Methoden

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
