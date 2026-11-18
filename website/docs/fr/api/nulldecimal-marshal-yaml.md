---
locale: fr
source_revision: 14c3a4a68a793992e79fa29a86d0ef2a5b8427f3e5ce4523962a0b1a25a66098
translation_provenance: en-US-french-reviewed
api_id: "NullDecimal::marshal_yaml"
---
# NullDecimal::marshal_yaml

Obtenir le contenu sémantique yaml.

```rust
fn marshal_yaml (& self) -> Option < String >
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie `Option<String>`/string | null. Rust exige la fonctionnalité yaml, activée dans le navigateur. Renvoie une chaîne scalaire YAML sémantique, pas un document YAML sérialisé. Transmettez-la comme chaîne à un générateur YAML pour que les guillemets et étiquettes soient correctement traités. Le texte décimal canonique supprime les zéros fractionnaires finaux. Une valeur nullable invalide renvoie None/null, qui représente un scalaire YAML null ; une valeur valide avec un zéro interne non initialisé renvoie "0". La lecture ne change jamais l’initialisation ni la validité ; le navigateur représente None de Rust par null.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-marshal-yaml" locale="fr" />

## Limites du navigateur

La source est limitée à 8192 octets UTF-8. Avant l’analyse numérique du cœur, Rust contrôle le premier scalaire YAML décodé ou le texte de l’élément XML, y compris les échappements, CDATA, la normalisation des traits de soulignement et l’insertion de zéros dans les flottants YAML. Les scalaires YAML acceptent au maximum 4096 caractères de chiffres hexadécimaux pour limiter l’allocation des entiers selon leur base ; les contrôles conservateurs peuvent rejeter des entrées acceptées nativement. L’échelle, le coefficient et le nombre d’opérations suivent la [politique de ressources](../resource-policy). Les handles déplacés ne peuvent plus être lus ni modifiés.

## Méthodes associées

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
