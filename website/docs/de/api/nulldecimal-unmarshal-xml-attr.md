---
locale: de
source_revision: f9048a3e7634cd109eea9581928de5e077f7c7324b6f3b621f732137009253fb
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::unmarshal_xml_attr"
---
# NullDecimal::unmarshal_xml_attr

xml_attr in einen vorhandenen Wert decodieren.

```rust
fn unmarshal_xml_attr (& mut self , value : & str) -> Result < () , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

value: &str/string; ändert den Empfänger und liefert `Result<(), DecimalError>`/undefined. Rust benötigt xml. Übergeben Sie den Attributwert mit bereits aufgelösten Entitäten, kein XML-Attributfragment: "1.23" ist gültig, aber `"&#49;.23"` wird nicht erneut decodiert. Entfernt äußeren Leerraum und parst exakten Dezimaltext. Leerer Inhalt löscht valid und setzt den inneren Decimal auf nicht initialisierte Null zurück. Erfolgreiches Decodieren einer Eingabe außer null initialisiert die Dezimalzahl und erhält nachgestellte Skala bei Zahlenwerten ungleich null. Setzt valid=true. Fehler erhalten alle Empfängerfelder und Bibliotheksfehlerarten einschließlich Unmarshal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-unmarshal-xml-attr" locale="de" />

## Verwandte Methoden

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
