---
locale: de
source_revision: 22a6d48f36f7716bfae657ccf09549a7c182a92cb0e3afef6f24a947535edb2b
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::impl Deserialize < 'de >"
---
# NullDecimal::impl Deserialize < 'de >

Mit dem generischen Serde-Visitor deserialisieren.

```rust
impl< 'de > Deserialize < 'de > for NullDecimal
```

## Parameter, Rückgabe und übliche Verwendung

deserializer: D mit Implementierung von Deserializer; liefert Self oder D::Error. Akzeptiert exakte Dezimalzeichenfolgen, ganzzahlige Visitor-Eingaben und null. Gleitkomma-Visitor-Eingaben werden abgewiesen: Eine JSON-Zahl mit Bruchteil/Exponent muss den speziellen rohen JSON-/config-Adapter verwenden. JSON null ergibt einen ungültigen nullable-Wert mit innerer nicht initialisierter Null. Leere Zeichenfolgen sind ungültig. Der statische Browseradapter deserialize(source) führt serde_json auf vollständigem ursprünglichem JSON-Text aus; sein Umfang ist enger als die Unterstützung beliebiger Rust-Deserializer. Fehler stellen category="serde" und die ursprüngliche Meldung bereit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-nulldecimal-impl-deserialize-de" locale="de" />

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
