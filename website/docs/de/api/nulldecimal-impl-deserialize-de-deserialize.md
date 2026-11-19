---
locale: de
source_revision: c31182c59d69a63bcf444ed429e3135dcff513f7463525a159b980c0e11afd3d
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::impl Deserialize < 'de >::deserialize"
---
# NullDecimal::impl Deserialize < 'de >::deserialize

Mit dem generischen Serde-Visitor deserialisieren.

```rust
fn deserialize < D : Deserializer < 'de > > (d : D) -> Result < Self , D :: Error >
```

## Parameter, Rückgabe und übliche Verwendung

deserializer: D mit Implementierung von Deserializer; liefert Self oder D::Error. Akzeptiert exakte Dezimalzeichenfolgen, ganzzahlige Visitor-Eingaben und null. Gleitkomma-Visitor-Eingaben werden abgewiesen: Eine JSON-Zahl mit Bruchteil/Exponent muss den speziellen rohen JSON-/config-Adapter verwenden. JSON null ergibt einen ungültigen nullable-Wert mit innerer nicht initialisierter Null. Leere Zeichenfolgen sind ungültig. Der statische Browseradapter deserialize(source) führt serde_json auf vollständigem ursprünglichem JSON-Text aus; sein Umfang ist enger als die Unterstützung beliebiger Rust-Deserializer. Fehler stellen category="serde" und die ursprüngliche Meldung bereit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-nulldecimal-impl-deserialize-de-deserialize" locale="de" />

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
