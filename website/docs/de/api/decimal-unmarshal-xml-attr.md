---
locale: de
source_revision: 8e284228528617403ea37afb52c965d21d076aee976dac5cd366e305507e8e05
translation_provenance: en-US-german-reviewed
api_id: "Decimal::unmarshal_xml_attr"
---
# Decimal::unmarshal_xml_attr

xml_attr in einen vorhandenen Wert decodieren.

```rust
fn unmarshal_xml_attr (& mut self , value : & str) -> Result < () , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

value: &str/string; ändert den Empfänger und liefert `Result<(), DecimalError>`/undefined. Rust benötigt xml. Übergeben Sie den Attributwert mit bereits aufgelösten Entitäten, kein XML-Attributfragment: "1.23" ist gültig, aber `"&#49;.23"` wird nicht erneut decodiert. Entfernt äußeren Leerraum und parst exakten Dezimaltext. Leerer Inhalt setzt den Empfänger auf nicht initialisierte Null zurück. Erfolgreiches Decodieren einer Eingabe außer null initialisiert die Dezimalzahl und erhält nachgestellte Skala bei Zahlenwerten ungleich null. Fehler erhalten alle Empfängerfelder und Bibliotheksfehlerarten einschließlich Unmarshal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-unmarshal-xml-attr" locale="de" />

## Verwandte Methoden

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
