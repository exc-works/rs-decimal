---
locale: fr
source_revision: 90631f865fb2217f0d8137591d3ab6e82b220d3c98120fa1ec15f91a894f2664
translation_provenance: en-US-french-reviewed
api_id: "Decimal::marshal_xml"
---
# Decimal::marshal_xml

Obtenir le contenu sémantique xml.

```rust
fn marshal_xml (& self) -> String
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie String/string. Rust exige la fonctionnalité xml, activée dans le navigateur. Renvoie le contenu textuel de l’élément ; l’appelant fournit le nom d’élément, l’espace de noms et l’échappement XML. Le texte décimal canonique supprime les zéros fractionnaires finaux. Un zéro non initialisé renvoie un contenu vide ; un zéro initialisé renvoie "0". La lecture ne change jamais l’initialisation ni la validité.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-marshal-xml" locale="fr" />

## Méthodes associées

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
