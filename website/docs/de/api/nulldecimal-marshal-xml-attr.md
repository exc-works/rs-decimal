---
locale: de
source_revision: cee1a90bda7ee828acf5a855ac4b338f7c02ca9ef05b0b3742b98db12c7cf78a
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

## Grenzen im Browser

Die Quelle ist auf 8192 UTF-8-Bytes begrenzt. Vor dem numerischen Kernparser prüft Rust den decodierten ersten YAML-Skalar bzw. XML-Elementtext, einschließlich Escapes, CDATA, Unterstrichnormalisierung und Nulleinfügung bei YAML-Gleitkommazahlen. YAML-Skalare erlauben höchstens 4096 hexadezimale Ziffernzeichen zur Begrenzung basisabhängiger Ganzzahlallokation; konservative Prüfungen können nativ gültige Eingaben ablehnen. Skala, Koeffizient und Operationszahl folgen der [Ressourcenrichtlinie](../resource-policy). Verschobene Handles können weder gelesen noch verändert werden.

## Verwandte Methoden

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
