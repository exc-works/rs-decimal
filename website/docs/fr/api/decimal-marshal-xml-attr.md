---
locale: fr
source_revision: 8ca020b2d56652a32165b4756446630d2a88be60a068a89e4f65ce218930abe5
translation_provenance: en-US-french-reviewed
api_id: "Decimal::marshal_xml_attr"
---
# Decimal::marshal_xml_attr

Obtenir le contenu sémantique xml_attr.

```rust
fn marshal_xml_attr (& self) -> String
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie String/string. Rust exige la fonctionnalité xml, activée dans le navigateur. Renvoie le contenu d’attribut déjà décodé ; l’appelant fournit le nom d’attribut et l’échappement XML. Le texte décimal canonique supprime les zéros fractionnaires finaux. Un zéro non initialisé renvoie un contenu vide ; un zéro initialisé renvoie "0". La lecture ne change jamais l’initialisation ni la validité.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-marshal-xml-attr" locale="fr" />

## Méthodes associées

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
