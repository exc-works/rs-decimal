---
locale: de
source_revision: e94c40441044730df649ba261de89daead4c5ce018d30a42e07e0ff558def35c
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::unmarshal_xml"
---
# NullDecimal::unmarshal_xml

xml in einen vorhandenen Wert decodieren.

```rust
fn unmarshal_xml (& mut self , source : & str) -> Result < () , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

source: &str/string; ändert den Empfänger und liefert `Result<(), DecimalError>`/undefined. Rust benötigt xml. Decodiert das erste XML-Element und kombiniert dessen direkten Text, CDATA und aufgelöste Entitätsreferenzen. Text verschachtelter Kinder wird ignoriert; Text vor/nach einem Kind wird zusammengefügt. Spätere Elemente werden ignoriert. Namen, Attribute, Zeichenreferenzen und passende Endtags validiert der Kern. Nach Entfernen äußeren Leerraums leerer Inhalt löscht valid und setzt den inneren Decimal auf nicht initialisierte Null zurück. Erfolgreiches Decodieren einer Eingabe außer null initialisiert die Dezimalzahl und erhält nachgestellte Skala bei Zahlenwerten ungleich null. Setzt valid=true. Fehler erhalten alle Empfängerfelder und Bibliotheksfehlerarten einschließlich Unmarshal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-unmarshal-xml" locale="de" />

## Verwandte Methoden

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
