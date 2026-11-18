---
locale: de
source_revision: e89c105d8071261190fddaa241c9f1fa55981badd4bbdc0f74617c9bd940eb39
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

## Grenzen im Browser

Die Quelle ist auf 8192 UTF-8-Bytes begrenzt. Vor dem numerischen Kernparser prüft Rust den decodierten ersten YAML-Skalar bzw. XML-Elementtext, einschließlich Escapes, CDATA, Unterstrichnormalisierung und Nulleinfügung bei YAML-Gleitkommazahlen. YAML-Skalare erlauben höchstens 4096 hexadezimale Ziffernzeichen zur Begrenzung basisabhängiger Ganzzahlallokation; konservative Prüfungen können nativ gültige Eingaben ablehnen. Skala, Koeffizient und Operationszahl folgen der [Ressourcenrichtlinie](../resource-policy). Verschobene Handles können weder gelesen noch verändert werden.

## Verwandte Methoden

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
