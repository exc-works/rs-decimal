---
locale: de
source_revision: 290824d81667633f4375180726adda2c851b5415686c46a9a13af6b626baaf4f
translation_provenance: en-US-german-reviewed
api_id: "Decimal::impl Deserialize < 'de >"
---
# Decimal::impl Deserialize < 'de >

Mit dem generischen Serde-Visitor deserialisieren.

```rust
impl< 'de > Deserialize < 'de > for Decimal
```

## Parameter, Rückgabe und übliche Verwendung

deserializer: D mit Implementierung von Deserializer; liefert Self oder D::Error. Akzeptiert exakte Dezimalzeichenfolgen, ganzzahlige Visitor-Eingaben und null. Gleitkomma-Visitor-Eingaben werden abgewiesen: Eine JSON-Zahl mit Bruchteil/Exponent muss den speziellen rohen JSON-/config-Adapter verwenden. JSON null ergibt nicht initialisierte Decimal-Null. Leere Zeichenfolgen sind ungültig. Der statische Browseradapter deserialize(source) führt serde_json auf vollständigem ursprünglichem JSON-Text aus; sein Umfang ist enger als die Unterstützung beliebiger Rust-Deserializer. Fehler stellen category="serde" und die ursprüngliche Meldung bereit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-decimal-impl-deserialize-de" locale="de" />

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
