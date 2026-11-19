---
locale: de
source_revision: 90631f865fb2217f0d8137591d3ab6e82b220d3c98120fa1ec15f91a894f2664
translation_provenance: en-US-german-reviewed
api_id: "Decimal::marshal_xml"
---
# Decimal::marshal_xml

Semantischen xml-Inhalt erhalten.

```rust
fn marshal_xml (& self) -> String
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert String/string. Rust benötigt das im Browser aktivierte Feature xml. Liefert den Zeicheninhalt des Elements; der Aufrufer ergänzt Elementname, Namensraum und XML-Escaping. Kanonischer Dezimaltext entfernt nachgestellte Nachkommanullen. Nicht initialisierte Null liefert leeren Inhalt; initialisierte Null liefert "0". Lesen ändert niemals Initialisierung oder Gültigkeit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-marshal-xml" locale="de" />

## Verwandte Methoden

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
