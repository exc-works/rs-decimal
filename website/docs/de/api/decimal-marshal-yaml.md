---
locale: de
source_revision: 25be20babe80f8d8b33e901842da879899c54ff8a78f97427ea80f9ca18a005d
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

## Verwandte Methoden

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
