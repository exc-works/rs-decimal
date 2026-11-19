---
locale: de
source_revision: 634992be26686e49442a948ce62a476a4bcc34db498b408945e4ad08ce9f7530
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::marshal_yaml"
---
# NullDecimal::marshal_yaml

Semantischen yaml-Inhalt erhalten.

```rust
fn marshal_yaml (& self) -> Option < String >
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert `Option<String>`/string | null. Rust benötigt das im Browser aktivierte Feature yaml. Liefert einen semantischen YAML-Zeichenfolgenskalar, kein serialisiertes YAML-Dokument. Übergeben Sie ihn als Zeichenfolge an einen YAML-Emitter, damit Anführungszeichen und Tags korrekt behandelt werden. Kanonischer Dezimaltext entfernt nachgestellte Nachkommanullen. Ungültige nullable-Werte liefern None/null als YAML-null-Skalar. Gültige Werte mit innerer nicht initialisierter Null liefern "0". Lesen ändert niemals Initialisierung oder Gültigkeit; der Browser bildet Rust-None auf null ab.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-marshal-yaml" locale="de" />

## Verwandte Methoden

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
