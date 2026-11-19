---
locale: de
source_revision: b8d3cac3d5f9251c9158f0efb63e99a5a338182fad355d14b4c984c245247f04
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::marshal_xml_attr"
---
# NullDecimal::marshal_xml_attr

Semantischen xml_attr-Inhalt erhalten.

```rust
fn marshal_xml_attr (& self) -> Option < String >
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert `Option<String>`/string | null. Rust benötigt das im Browser aktivierte Feature xml. Liefert bereits decodierten Attributinhalt; der Aufrufer ergänzt Attributname und XML-Escaping. Kanonischer Dezimaltext entfernt nachgestellte Nachkommanullen. Ungültige nullable-Werte liefern None/null: Lassen Sie das Attribut weg, statt ein leeres Attribut zu schreiben. Gültige Werte mit innerer nicht initialisierter Null liefern "0". Lesen ändert niemals Initialisierung oder Gültigkeit; der Browser bildet Rust-None auf null ab.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-marshal-xml-attr" locale="de" />

## Verwandte Methoden

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
