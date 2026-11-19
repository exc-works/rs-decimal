---
locale: de
source_revision: 8ca020b2d56652a32165b4756446630d2a88be60a068a89e4f65ce218930abe5
translation_provenance: en-US-german-reviewed
api_id: "Decimal::marshal_xml_attr"
---
# Decimal::marshal_xml_attr

Semantischen xml_attr-Inhalt erhalten.

```rust
fn marshal_xml_attr (& self) -> String
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert String/string. Rust benötigt das im Browser aktivierte Feature xml. Liefert bereits decodierten Attributinhalt; der Aufrufer ergänzt Attributname und XML-Escaping. Kanonischer Dezimaltext entfernt nachgestellte Nachkommanullen. Nicht initialisierte Null liefert leeren Inhalt; initialisierte Null liefert "0". Lesen ändert niemals Initialisierung oder Gültigkeit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-marshal-xml-attr" locale="de" />

## Verwandte Methoden

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
