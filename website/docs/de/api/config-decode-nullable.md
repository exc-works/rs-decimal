---
locale: de
source_revision: 324990894f6327d9f0e88937d867ea0f32bc3c792ba37210279bca0b73495f73
translation_provenance: en-US-german-reviewed
api_id: "config::decode_nullable"
---
# config::decode_nullable

Einen optionalen Konfigurationswert decodieren.

```rust
fn decode_nullable (value : ConfigValue < '_ >) -> Result < NullDecimal , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

value: ConfigValue; liefert `Result<NullDecimal, DecimalError>`/ein neues nullable-Handle. Null, exakt leeres Text und leeres Bytes werden ungültig mit innerer nicht initialisierter Null. Text nur aus Leerraum und leeres Number scheitern weiterhin beim Dezimalparsen. Vorhandene Nullable klonen Gültigkeit und verborgene innere Daten auch bei Ungültigkeit; ein ausgeliehener Decimal wird gültig, ohne Initialisierung zu erzwingen. Andere Eingaben folgen decode_decimal und werden bei Erfolg gültig. Eingaben werden nicht konsumiert.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-decode-nullable" locale="de" />

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
