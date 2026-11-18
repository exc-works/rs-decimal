---
locale: fr
source_revision: e89c105d8071261190fddaa241c9f1fa55981badd4bbdc0f74617c9bd940eb39
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

## Limites du navigateur

La source est limitée à 8192 octets UTF-8. Avant l’analyse numérique du cœur, Rust contrôle le premier scalaire YAML décodé ou le texte de l’élément XML, y compris les échappements, CDATA, la normalisation des traits de soulignement et l’insertion de zéros dans les flottants YAML. Les scalaires YAML acceptent au maximum 4096 caractères de chiffres hexadécimaux pour limiter l’allocation des entiers selon leur base ; les contrôles conservateurs peuvent rejeter des entrées acceptées nativement. L’échelle, le coefficient et le nombre d’opérations suivent la [politique de ressources](../resource-policy). Les handles déplacés ne peuvent plus être lus ni modifiés.

## Méthodes associées

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
