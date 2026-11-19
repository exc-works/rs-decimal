---
locale: de
source_revision: fd8204204097a2ca3064943a4cf9e6b865ae769705e2693bc4a79f71be2a4582
translation_provenance: en-US-german-reviewed
api_id: "Decimal::impl Serialize"
---
# Decimal::impl Serialize

Über den Rust-Serde-Trait serialisieren.

```rust
impl Serialize for Decimal
```

## Parameter, Rückgabe und übliche Verwendung

serializer: S mit Implementierung von Serializer; liefert S::Ok/S::Error. Der Kern serialisiert kanonische Dezimalzeichenfolgen oder null. Nicht initialisierter Decimal wird als null serialisiert; initialisierte Null als Zeichenfolge "0". Nachgestellte Nachkommanullen werden entfernt. Der Browseradapter serialize() wählt serde_json und liefert vollständigen JSON-Text. Beliebige Rust-Serializer werden nicht bereitgestellt.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-decimal-impl-serialize" locale="de" />

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
