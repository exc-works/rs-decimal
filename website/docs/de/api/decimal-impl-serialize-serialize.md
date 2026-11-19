---
locale: de
source_revision: b7aac8fb7a35df9ddf10adc50d88fb1bfd0169290aa22d5f4ad96531651b49c8
translation_provenance: en-US-german-reviewed
api_id: "Decimal::impl Serialize::serialize"
---
# Decimal::impl Serialize::serialize

Über den Rust-Serde-Trait serialisieren.

```rust
fn serialize < S : Serializer > (& self , s : S) -> Result < S :: Ok , S :: Error >
```

## Parameter, Rückgabe und übliche Verwendung

serializer: S mit Implementierung von Serializer; liefert S::Ok/S::Error. Der Kern serialisiert kanonische Dezimalzeichenfolgen oder null. Nicht initialisierter Decimal wird als null serialisiert; initialisierte Null als Zeichenfolge "0". Nachgestellte Nachkommanullen werden entfernt. Der Browseradapter serialize() wählt serde_json und liefert vollständigen JSON-Text. Beliebige Rust-Serializer werden nicht bereitgestellt.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-decimal-impl-serialize-serialize" locale="de" />

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
