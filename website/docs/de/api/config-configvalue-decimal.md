---
locale: de
source_revision: 8d101e011ba2e4f441ebfc6371c73cb22589c6224599cc87e367af949b20dc8f
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue::Decimal"
---
# config::ConfigValue::Decimal

Konfigurationseingabe Decimal wählen.

```rust
Decimal (& 'a Decimal)
```

## Parameter, Rückgabe und übliche Verwendung

Einen vorhandenen Decimal ausleihen und beim Decodieren Koeffizient, Skala und Initialisierung klonen. Die Quelle bleibt nutzbar. Freigeben/Verschieben der Browserquelle vor dem Decodieren macht die ausgeliehene Variante ungültig.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-decimal" locale="de" />

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
