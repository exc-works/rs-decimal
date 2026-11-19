---
locale: de
source_revision: 3594148ee83ab0bf68ef800a2a73612702e5a4d6ff495df20f526a7cdddcfcf1
translation_provenance: en-US-german-reviewed
api_id: "config::deserialize_json_decimal"
---
# config::deserialize_json_decimal

Ein exaktes JSON-Konfigurationsfeld deserialisieren.

```rust
fn deserialize_json_decimal < 'de , D : Deserializer < 'de > > (deserializer : D ,) -> Result < Decimal , D :: Error >
```

## Parameter, Rückgabe und übliche Verwendung

deserializer: D mit Implementierung von Serde Deserializer; vorgesehen für #[serde(deserialize_with = "rs_decimal::config::deserialize_json_decimal")]. Benötigt eine serde_json-Quelle und das Feature config. Erhält ursprüngliche Zahlentoken ohne Gleitkommaumweg. Der Browser stellt einen konkreten Adapter für eine vollständige JSON-Skalarquellzeichenfolge bereit, keinen beliebigen Deserializer. JSON null und leere Zeichenfolgen werden für das nicht-nullable-Ziel abgewiesen. Fehler haben den Serde-Deserializer-Fehlertyp; Browser-category="serde" erhält dessen Meldung, statt DecimalError-Arten zu erfinden. Nachgestellte Eingabe außer JSON-Leerraum wird abgewiesen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-deserialize-json-decimal" locale="de" />

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
