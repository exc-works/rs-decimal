---
locale: de
source_revision: 7b63356b05d421377bb8ec0a29f08c338eaab7596c00a0c507a722a2de09d089
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::marshal_xml"
---
# NullDecimal::marshal_xml

Semantischen xml-Inhalt erhalten.

```rust
fn marshal_xml (& self) -> String
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert String/string. Rust benötigt das im Browser aktivierte Feature xml. Liefert den Zeicheninhalt des Elements; der Aufrufer ergänzt Elementname, Namensraum und XML-Escaping. Kanonischer Dezimaltext entfernt nachgestellte Nachkommanullen. Ungültige nullable-Werte liefern leeren Inhalt; gültige Werte mit innerer nicht initialisierter Null liefern "0". Lesen ändert niemals Initialisierung oder Gültigkeit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-marshal-xml" locale="de" />

## Verwandte Methoden

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
