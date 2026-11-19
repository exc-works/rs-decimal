---
locale: de
source_revision: 9fa4dc63685c8b7863c54238c3b1bc5377c717d45438360151595cb826deb45d
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::marshal_bson_value"
---
# NullDecimal::marshal_bson_value

Ein BSON-Typbyte und eine Skalar-Payload codieren.

```rust
fn marshal_bson_value (& self) -> Result < (u8 , Vec < u8 >) , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert `Result<(u8, Vec<u8>), DecimalError>`/[number, Uint8Array]. Rust benötigt das im Browserbuild aktivierte Feature bson. Das Tupel enthält einen rohen BSON-Skalar, kein vollständiges BSON-Dokument. Gültige nullable-Werte werden als BSON-Zeichenfolgen codiert, auch bei innerer nicht initialisierter Null. Ungültige Werte werden Typ 10 mit leerer Payload. Die Zeichenfolgen-Payload enthält eine Little-Endian-i32-Bytelänge einschließlich abschließendem NUL, danach kanonischen Dezimaltext ohne nachgestellte Nachkommanullen. Die zurückgegebenen Payload-Bytes sind eine unabhängige Kopie; Codieren ändert weder Initialisierung noch Gültigkeit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-nulldecimal-marshal-bson-value" locale="de" />

## Verwandte Methoden

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
