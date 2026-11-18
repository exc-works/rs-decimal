---
locale: de
source_revision: e1a6c7014c208ee06566822a33e4d32bdbca21d8076f5cc955ee91bcd5ae501d
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::unmarshal_yaml"
---
# NullDecimal::unmarshal_yaml

yaml in einen vorhandenen Wert decodieren.

```rust
fn unmarshal_yaml (& mut self , source : & str) -> Result < () , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

source: &str/string; ändert den Empfänger und liefert `Result<(), DecimalError>`/undefined. Rust benötigt yaml. Übergeben Sie ursprüngliche YAML-Quelle, damit Anführungszeichen, Tags und exakte Ziffern für Rust erhalten bleiben. Unzitiertes 012 wird oktal zu 10 aufgelöst; zitiertes "012" oder !!str 012 wird dezimal als 12 geparst. Numerische Tags/Basen/Unterstriche folgen dem Kernresolver: unzitiertes 1_000 ist 1000, zitiertes "1_000" ist ungültig. Boolesche Werte, Sammlungen und Blockskalare werden abgewiesen. Nur das erste Dokument liefert den Wert; spätere Dokumente müssen trotzdem syntaktisch gültig sein. Null oder ein leeres Dokument löscht valid und setzt den inneren Decimal auf nicht initialisierte Null zurück. Erfolgreiches Decodieren einer Eingabe außer null initialisiert die Dezimalzahl und erhält nachgestellte Skala bei Zahlenwerten ungleich null. Setzt valid=true. Fehler erhalten alle Empfängerfelder und Bibliotheksfehlerarten einschließlich Unmarshal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-unmarshal-yaml" locale="de" />

## Grenzen im Browser

Die Quelle ist auf 8192 UTF-8-Bytes begrenzt. Vor dem numerischen Kernparser prüft Rust den decodierten ersten YAML-Skalar bzw. XML-Elementtext, einschließlich Escapes, CDATA, Unterstrichnormalisierung und Nulleinfügung bei YAML-Gleitkommazahlen. YAML-Skalare erlauben höchstens 4096 hexadezimale Ziffernzeichen zur Begrenzung basisabhängiger Ganzzahlallokation; konservative Prüfungen können nativ gültige Eingaben ablehnen. Skala, Koeffizient und Operationszahl folgen der [Ressourcenrichtlinie](../resource-policy). Verschobene Handles können weder gelesen noch verändert werden.

## Verwandte Methoden

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
