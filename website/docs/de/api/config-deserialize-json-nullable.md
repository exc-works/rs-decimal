---
locale: de
source_revision: 79f19f9f6f20dd385b69becd2b7e8e1abfeb536b960dc7d8b240d8301a6064de
translation_provenance: en-US-german-reviewed
api_id: "config::deserialize_json_nullable"
---
# config::deserialize_json_nullable

Ein exaktes JSON-Konfigurationsfeld deserialisieren.

```rust
fn deserialize_json_nullable < 'de , D : Deserializer < 'de > > (deserializer : D ,) -> Result < NullDecimal , D :: Error >
```

## Parameter, Rückgabe und übliche Verwendung

deserializer: D mit Implementierung von Serde Deserializer; vorgesehen für #[serde(deserialize_with = "rs_decimal::config::deserialize_json_nullable")]. Benötigt eine serde_json-Quelle und das Feature config. Erhält ursprüngliche Zahlentoken ohne Gleitkommaumweg. Der Browser stellt einen konkreten Adapter für eine vollständige JSON-Skalarquellzeichenfolge bereit, keinen beliebigen Deserializer. JSON null und exakt leere JSON-Zeichenfolge werden ungültig; Zeichenfolgen nur aus Leerraum scheitern weiterhin. Erfolgreiche nichtleere Werte werden gültig. Fehler haben den Serde-Deserializer-Fehlertyp; Browser-category="serde" erhält dessen Meldung, statt DecimalError-Arten zu erfinden. Nachgestellte Eingabe außer JSON-Leerraum wird abgewiesen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-deserialize-json-nullable" locale="de" />

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
