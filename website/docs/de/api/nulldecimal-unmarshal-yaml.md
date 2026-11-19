---
locale: de
source_revision: dbc15c20de89c4c0e7f4a73778b0f1b410d8c87e2c8de57f1e13798c66980cb2
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

## Verwandte Methoden

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
