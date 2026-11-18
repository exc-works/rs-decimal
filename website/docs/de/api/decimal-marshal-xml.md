---
locale: de
source_revision: 5f41d4af42d404ec01165b3190db1cd2f96b9a84fa765bafbce34ae6b76aa2ef
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

## Grenzen im Browser

Die Quelle ist auf 8192 UTF-8-Bytes begrenzt. Vor dem numerischen Kernparser prüft Rust den decodierten ersten YAML-Skalar bzw. XML-Elementtext, einschließlich Escapes, CDATA, Unterstrichnormalisierung und Nulleinfügung bei YAML-Gleitkommazahlen. YAML-Skalare erlauben höchstens 4096 hexadezimale Ziffernzeichen zur Begrenzung basisabhängiger Ganzzahlallokation; konservative Prüfungen können nativ gültige Eingaben ablehnen. Skala, Koeffizient und Operationszahl folgen der [Ressourcenrichtlinie](../resource-policy). Verschobene Handles können weder gelesen noch verändert werden.

## Verwandte Methoden

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
