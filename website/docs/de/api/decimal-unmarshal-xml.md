---
locale: de
source_revision: 1640bba425b4f674d29468bfc500f4a4319c227e5d4c34f98666dafad0d9b981
translation_provenance: en-US-german-reviewed
api_id: "Decimal::unmarshal_xml"
---
# Decimal::unmarshal_xml

xml in einen vorhandenen Wert decodieren.

```rust
fn unmarshal_xml (& mut self , source : & str) -> Result < () , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

source: &str/string; ändert den Empfänger und liefert `Result<(), DecimalError>`/undefined. Rust benötigt xml. Decodiert das erste XML-Element und kombiniert dessen direkten Text, CDATA und aufgelöste Entitätsreferenzen. Text verschachtelter Kinder wird ignoriert; Text vor/nach einem Kind wird zusammengefügt. Spätere Elemente werden ignoriert. Namen, Attribute, Zeichenreferenzen und passende Endtags validiert der Kern. Nach Entfernen äußeren Leerraums leerer Inhalt setzt den Empfänger auf nicht initialisierte Null zurück; YAML null lässt ihn dagegen unverändert. Erfolgreiches Decodieren einer Eingabe außer null initialisiert die Dezimalzahl und erhält nachgestellte Skala bei Zahlenwerten ungleich null. Fehler erhalten alle Empfängerfelder und Bibliotheksfehlerarten einschließlich Unmarshal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-unmarshal-xml" locale="de" />

## Grenzen im Browser

Die Quelle ist auf 8192 UTF-8-Bytes begrenzt. Vor dem numerischen Kernparser prüft Rust den decodierten ersten YAML-Skalar bzw. XML-Elementtext, einschließlich Escapes, CDATA, Unterstrichnormalisierung und Nulleinfügung bei YAML-Gleitkommazahlen. YAML-Skalare erlauben höchstens 4096 hexadezimale Ziffernzeichen zur Begrenzung basisabhängiger Ganzzahlallokation; konservative Prüfungen können nativ gültige Eingaben ablehnen. Skala, Koeffizient und Operationszahl folgen der [Ressourcenrichtlinie](../resource-policy). Verschobene Handles können weder gelesen noch verändert werden.

## Verwandte Methoden

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
