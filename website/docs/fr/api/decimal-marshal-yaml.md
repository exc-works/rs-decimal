---
locale: fr
source_revision: 2596ef8441aa7ec726abb87c7af62732ec4f2729da79754eba71b3cb01735596
translation_provenance: en-US-french-reviewed
api_id: "Decimal::marshal_yaml"
---
# Decimal::marshal_yaml

Obtenir le contenu sémantique yaml.

```rust
fn marshal_yaml (& self) -> String
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie String/string. Rust exige la fonctionnalité yaml, activée dans le navigateur. Renvoie une chaîne scalaire YAML sémantique, pas un document YAML sérialisé. Transmettez-la comme chaîne à un générateur YAML pour que les guillemets et étiquettes soient correctement traités. Le texte décimal canonique supprime les zéros fractionnaires finaux. Le zéro par défaut non initialisé renvoie "0". La lecture ne change jamais l’initialisation ni la validité.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-marshal-yaml" locale="fr" />

## Limites du navigateur

La source est limitée à 8192 octets UTF-8. Avant l’analyse numérique du cœur, Rust contrôle le premier scalaire YAML décodé ou le texte de l’élément XML, y compris les échappements, CDATA, la normalisation des traits de soulignement et l’insertion de zéros dans les flottants YAML. Les scalaires YAML acceptent au maximum 4096 caractères de chiffres hexadécimaux pour limiter l’allocation des entiers selon leur base ; les contrôles conservateurs peuvent rejeter des entrées acceptées nativement. L’échelle, le coefficient et le nombre d’opérations suivent la [politique de ressources](../resource-policy). Les handles déplacés ne peuvent plus être lus ni modifiés.

## Méthodes associées

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
