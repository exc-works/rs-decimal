---
locale: de
source_revision: 14c3a4a68a793992e79fa29a86d0ef2a5b8427f3e5ce4523962a0b1a25a66098
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::marshal_yaml"
---
# NullDecimal::marshal_yaml

Semantischen yaml-Inhalt erhalten.

```rust
fn marshal_yaml (& self) -> Option < String >
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert `Option<String>`/string | null. Rust benötigt das im Browser aktivierte Feature yaml. Liefert einen semantischen YAML-Zeichenfolgenskalar, kein serialisiertes YAML-Dokument. Übergeben Sie ihn als Zeichenfolge an einen YAML-Emitter, damit Anführungszeichen und Tags korrekt behandelt werden. Kanonischer Dezimaltext entfernt nachgestellte Nachkommanullen. Ungültige nullable-Werte liefern None/null als YAML-null-Skalar. Gültige Werte mit innerer nicht initialisierter Null liefern "0". Lesen ändert niemals Initialisierung oder Gültigkeit; der Browser bildet Rust-None auf null ab.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-marshal-yaml" locale="de" />

## Grenzen im Browser

Die Quelle ist auf 8192 UTF-8-Bytes begrenzt. Vor dem numerischen Kernparser prüft Rust den decodierten ersten YAML-Skalar bzw. XML-Elementtext, einschließlich Escapes, CDATA, Unterstrichnormalisierung und Nulleinfügung bei YAML-Gleitkommazahlen. YAML-Skalare erlauben höchstens 4096 hexadezimale Ziffernzeichen zur Begrenzung basisabhängiger Ganzzahlallokation; konservative Prüfungen können nativ gültige Eingaben ablehnen. Skala, Koeffizient und Operationszahl folgen der [Ressourcenrichtlinie](../resource-policy). Verschobene Handles können weder gelesen noch verändert werden.

## Verwandte Methoden

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
