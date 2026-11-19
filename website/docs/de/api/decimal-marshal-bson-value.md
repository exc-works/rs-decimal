---
locale: de
source_revision: 44afbeff0d59cbde33e83e7f33c7e22c9903950f7ecf43d6f8230c6cde4b2ec4
translation_provenance: en-US-german-reviewed
api_id: "Decimal::marshal_bson_value"
---
# Decimal::marshal_bson_value

Ein BSON-Typbyte und eine Skalar-Payload codieren.

```rust
fn marshal_bson_value (& self) -> Result < (u8 , Vec < u8 >) , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert `Result<(u8, Vec<u8>), DecimalError>`/[number, Uint8Array]. Rust benötigt das im Browserbuild aktivierte Feature bson. Das Tupel enthält einen rohen BSON-Skalar, kein vollständiges BSON-Dokument. Initialisierte Werte werden als BSON-Zeichenfolgen (Typ 2), nicht Decimal128 codiert. Nicht initialisierte Null wird Typ 10 mit leerer Payload. Die Zeichenfolgen-Payload enthält eine Little-Endian-i32-Bytelänge einschließlich abschließendem NUL, danach kanonischen Dezimaltext ohne nachgestellte Nachkommanullen. Die zurückgegebenen Payload-Bytes sind eine unabhängige Kopie; Codieren ändert weder Initialisierung noch Gültigkeit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-marshal-bson-value" locale="de" />

## Verwandte Methoden

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
