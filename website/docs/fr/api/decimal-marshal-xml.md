---
locale: fr
source_revision: 5f41d4af42d404ec01165b3190db1cd2f96b9a84fa765bafbce34ae6b76aa2ef
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

## Limites du navigateur

La source est limitée à 8192 octets UTF-8. Avant l’analyse numérique du cœur, Rust contrôle le premier scalaire YAML décodé ou le texte de l’élément XML, y compris les échappements, CDATA, la normalisation des traits de soulignement et l’insertion de zéros dans les flottants YAML. Les scalaires YAML acceptent au maximum 4096 caractères de chiffres hexadécimaux pour limiter l’allocation des entiers selon leur base ; les contrôles conservateurs peuvent rejeter des entrées acceptées nativement. L’échelle, le coefficient et le nombre d’opérations suivent la [politique de ressources](../resource-policy). Les handles déplacés ne peuvent plus être lus ni modifiés.

## Méthodes associées

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
