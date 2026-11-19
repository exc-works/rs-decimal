---
locale: de
source_revision: 5f7a180e14ebabfd6785a42604765a6ece20be12513e46794eb1f0299f911006
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::string"
---
# NullDecimal::string

Kanonischen nullable-Text lesen.

```rust
fn string (& self) -> String
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert String/string. Gültige Werte verwenden den kanonischen Text des inneren Decimal ohne nachgestellte Nachkommanullen. Ungültige Werte liefern unabhängig von verborgenen inneren Daten das Literal null. Dies unterscheidet sich von marshal_text, das bei ungültigen Werten leere Bytes liefert. Lesen ändert weder Gültigkeit noch Initialisierung.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-string" locale="de" />

## Verwandte Methoden

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
