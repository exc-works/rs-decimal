---
locale: fr
source_revision: 7b63356b05d421377bb8ec0a29f08c338eaab7596c00a0c507a722a2de09d089
translation_provenance: en-US-french-reviewed
api_id: "NullDecimal::marshal_xml"
---
# NullDecimal::marshal_xml

Obtenir le contenu sémantique xml.

```rust
fn marshal_xml (& self) -> String
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie String/string. Rust exige la fonctionnalité xml, activée dans le navigateur. Renvoie le contenu textuel de l’élément ; l’appelant fournit le nom d’élément, l’espace de noms et l’échappement XML. Le texte décimal canonique supprime les zéros fractionnaires finaux. Une valeur nullable invalide renvoie un contenu vide ; une valeur valide avec un zéro interne non initialisé renvoie "0". La lecture ne change jamais l’initialisation ni la validité.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-marshal-xml" locale="fr" />

## Méthodes associées

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
