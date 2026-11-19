---
locale: de
source_revision: 87ff1c64a9a30b8c362e1bf8438b7f751aeb4f561a2bd311095a9f2b86ed45db
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

## Verwandte Methoden

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
