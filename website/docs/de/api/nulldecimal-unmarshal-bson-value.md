---
locale: de
source_revision: 59bfa6e69fb4720796373509ad0a7f4325fcfd6dbb9c49ba19604b5d730cd4a1
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::unmarshal_bson_value"
---
# NullDecimal::unmarshal_bson_value

Einen BSON-Skalar in einen vorhandenen Wert decodieren.

```rust
fn unmarshal_bson_value (& mut self , kind : u8 , data : & [u8]) -> Result < () , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

kind: u8/number, data: &[u8]/Uint8Array; ändert den Empfänger und liefert `Result<(), DecimalError>`/undefined. Rust benötigt bson. Unterstützt double (1), string (2), null (10), int32 (16), int64 (18) und Decimal128 (19); numerische Payloads verwenden BSON-Little-Endian-Bytes. Übergeben Sie int64 als rohe Bytes, um Genauigkeitsverlust durch JS number zu vermeiden. BSON null löscht valid und setzt den inneren Decimal auf nicht initialisierte Null zurück. Erfolgreiche nullable-Decodierung einer Eingabe außer null setzt valid=true. Nicht unterstützte Arten, abgeschnittene Daten und nicht endliche Zahlen scheitern ohne Teiländerung. Kompatibilitätsdetails: Zeichenfolgendecodierung überspringt das deklarierte letzte Byte ohne NUL-Prüfung; Bytes hinter der benötigten Payload werden ignoriert. Decimal128 wird vom Rust-Kern decodiert und behält unterstützte Dezimalgenauigkeit; marshal_bson_value gibt weiterhin eine Zeichenfolge aus.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-nulldecimal-unmarshal-bson-value" locale="de" />

## Verwandte Methoden

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
