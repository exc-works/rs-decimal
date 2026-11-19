---
locale: de
source_revision: 8a589312845f2d23d85f3d6da1987ee87f79dfecdc97a705386e9a336b17d7ed
translation_provenance: en-US-german-reviewed
api_id: "Decimal::unmarshal_binary"
---
# Decimal::unmarshal_binary

Eine Dezimalzahl aus einem Go-kompatiblen Binärskalar ersetzen.

```rust
fn unmarshal_binary (& mut self , bytes : & [u8]) -> Result < () , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

bytes: &[u8]/Uint8Array; ändert den Empfänger und liefert `Result<(), DecimalError>`/undefined. Die ersten vier Bytes codieren eine Big-Endian-Skala, gefolgt von Gob-Header 2 (positiv) oder 3 (negativ) und Absolutwert. Erfolgreiches Decodieren erhält die codierte Skala. Leere Eingabe erzeugt initialisierte Null, anders als die nicht initialisierte Standardnull. Ein vier Byte langes Präfix ohne Gob-Payload wird ebenfalls als initialisierte Null mit dieser Skala akzeptiert. Kurze Präfixe, ungültige Versionen und Skalen über 131072 liefern Unmarshal-Kernfehler. Fehler erhalten den Empfänger.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-unmarshal-binary" locale="de" />

## Verwandte Methoden

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
