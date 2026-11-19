---
locale: de
source_revision: c7dd2a0f7387f7110e7a059f6333707931d18a8d6fce4c06641505cdcb037d58
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::impl Serialize::serialize"
---
# NullDecimal::impl Serialize::serialize

Über den Rust-Serde-Trait serialisieren.

```rust
fn serialize < S : Serializer > (& self , s : S) -> Result < S :: Ok , S :: Error >
```

## Parameter, Rückgabe und übliche Verwendung

serializer: S mit Implementierung von Serializer; liefert S::Ok/S::Error. Der Kern serialisiert kanonische Dezimalzeichenfolgen oder null. Ungültige nullable-Werte werden als null serialisiert; gültige Werte mit innerer nicht initialisierter Null als Zeichenfolge "0". Nachgestellte Nachkommanullen werden entfernt. Der Browseradapter serialize() wählt serde_json und liefert vollständigen JSON-Text. Beliebige Rust-Serializer werden nicht bereitgestellt.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-nulldecimal-impl-serialize-serialize" locale="de" />

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
