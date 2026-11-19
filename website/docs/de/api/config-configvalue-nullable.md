---
locale: de
source_revision: 5a5d502f0868797711c2e17ee061bb453dff465719822a493b3a33d226cf87b4
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue::Nullable"
---
# config::ConfigValue::Nullable

Konfigurationseingabe Nullable wählen.

```rust
Nullable (& 'a NullDecimal)
```

## Parameter, Rückgabe und übliche Verwendung

Einen vorhandenen nullable-Wert ausleihen. decode_nullable klont Gültigkeit und verborgene Daten; decode_decimal akzeptiert nur gültige Werte und klont deren inneren Decimal. Änderungen vor dem Decodieren werden berücksichtigt.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-nullable" locale="de" />

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
