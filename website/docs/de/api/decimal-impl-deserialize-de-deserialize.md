---
locale: de
source_revision: 24e00c21cccbe1d37441ecf7320c640c608396a025aad09f2a518e2a7e914703
translation_provenance: en-US-german-reviewed
api_id: "Decimal::impl Deserialize < 'de >::deserialize"
---
# Decimal::impl Deserialize < 'de >::deserialize

Mit dem generischen Serde-Visitor deserialisieren.

```rust
fn deserialize < D : Deserializer < 'de > > (d : D) -> Result < Self , D :: Error >
```

## Parameter, Rückgabe und übliche Verwendung

deserializer: D mit Implementierung von Deserializer; liefert Self oder D::Error. Akzeptiert exakte Dezimalzeichenfolgen, ganzzahlige Visitor-Eingaben und null. Gleitkomma-Visitor-Eingaben werden abgewiesen: Eine JSON-Zahl mit Bruchteil/Exponent muss den speziellen rohen JSON-/config-Adapter verwenden. JSON null ergibt nicht initialisierte Decimal-Null. Leere Zeichenfolgen sind ungültig. Der statische Browseradapter deserialize(source) führt serde_json auf vollständigem ursprünglichem JSON-Text aus; sein Umfang ist enger als die Unterstützung beliebiger Rust-Deserializer. Fehler stellen category="serde" und die ursprüngliche Meldung bereit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-decimal-impl-deserialize-de-deserialize" locale="de" />

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
