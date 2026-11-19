---
locale: de
source_revision: bfcaf340b0c1c1671610ce3a1025921897c59afd9bdf91da4be0b86f5f4d7446
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue::Bytes"
---
# config::ConfigValue::Bytes

Konfigurationseingabe Bytes wählen.

```rust
Bytes (& 'a [u8])
```

## Parameter, Rückgabe und übliche Verwendung

Bytes mit Dezimaltext ausleihen, keine binäre Dezimal-Payload. Änderungen am Browser-Uint8Array vor dem Decodieren werden berücksichtigt; Decodieren verändert das Array nie. Ungültiges UTF-8 scheitert beim Dezimalparsen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-bytes" locale="de" />

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
