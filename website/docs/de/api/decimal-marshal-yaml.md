---
locale: de
source_revision: 2596ef8441aa7ec726abb87c7af62732ec4f2729da79754eba71b3cb01735596
translation_provenance: en-US-german-reviewed
api_id: "Decimal::marshal_yaml"
---
# Decimal::marshal_yaml

Semantischen yaml-Inhalt erhalten.

```rust
fn marshal_yaml (& self) -> String
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert String/string. Rust benötigt das im Browser aktivierte Feature yaml. Liefert einen semantischen YAML-Zeichenfolgenskalar, kein serialisiertes YAML-Dokument. Übergeben Sie ihn als Zeichenfolge an einen YAML-Emitter, damit Anführungszeichen und Tags korrekt behandelt werden. Kanonischer Dezimaltext entfernt nachgestellte Nachkommanullen. Nicht initialisierte Standardnull liefert "0". Lesen ändert niemals Initialisierung oder Gültigkeit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-marshal-yaml" locale="de" />

## Grenzen im Browser

Die Quelle ist auf 8192 UTF-8-Bytes begrenzt. Vor dem numerischen Kernparser prüft Rust den decodierten ersten YAML-Skalar bzw. XML-Elementtext, einschließlich Escapes, CDATA, Unterstrichnormalisierung und Nulleinfügung bei YAML-Gleitkommazahlen. YAML-Skalare erlauben höchstens 4096 hexadezimale Ziffernzeichen zur Begrenzung basisabhängiger Ganzzahlallokation; konservative Prüfungen können nativ gültige Eingaben ablehnen. Skala, Koeffizient und Operationszahl folgen der [Ressourcenrichtlinie](../resource-policy). Verschobene Handles können weder gelesen noch verändert werden.

## Verwandte Methoden

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
