---
locale: de
source_revision: c4ab958e126151b59020a16b1317b5deb3282029f32339008d09669426a6a46f
translation_provenance: en-US-german-reviewed
api_id: "Decimal::unmarshal_yaml"
---
# Decimal::unmarshal_yaml

yaml in einen vorhandenen Wert decodieren.

```rust
fn unmarshal_yaml (& mut self , source : & str) -> Result < () , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

source: &str/string; ändert den Empfänger und liefert `Result<(), DecimalError>`/undefined. Rust benötigt yaml. Übergeben Sie ursprüngliche YAML-Quelle, damit Anführungszeichen, Tags und exakte Ziffern für Rust erhalten bleiben. Unzitiertes 012 wird oktal zu 10 aufgelöst; zitiertes "012" oder !!str 012 wird dezimal als 12 geparst. Numerische Tags/Basen/Unterstriche folgen dem Kernresolver: unzitiertes 1_000 ist 1000, zitiertes "1_000" ist ungültig. Boolesche Werte, Sammlungen und Blockskalare werden abgewiesen. Nur das erste Dokument liefert den Wert; spätere Dokumente müssen trotzdem syntaktisch gültig sein. Null oder ein leeres Dokument lässt den vorhandenen Empfänger einschließlich Initialisierung und Skala unverändert. Erfolgreiches Decodieren einer Eingabe außer null initialisiert die Dezimalzahl und erhält nachgestellte Skala bei Zahlenwerten ungleich null. Fehler erhalten alle Empfängerfelder und Bibliotheksfehlerarten einschließlich Unmarshal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-unmarshal-yaml" locale="de" />

## Verwandte Methoden

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
