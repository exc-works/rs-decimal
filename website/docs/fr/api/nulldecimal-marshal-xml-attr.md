---
locale: fr
source_revision: cee1a90bda7ee828acf5a855ac4b338f7c02ca9ef05b0b3742b98db12c7cf78a
translation_provenance: en-US-french-reviewed
api_id: "NullDecimal::marshal_xml_attr"
---
# NullDecimal::marshal_xml_attr

Obtenir le contenu sémantique xml_attr.

```rust
fn marshal_xml_attr (& self) -> Option < String >
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie `Option<String>`/string | null. Rust exige la fonctionnalité xml, activée dans le navigateur. Renvoie le contenu d’attribut déjà décodé ; l’appelant fournit le nom d’attribut et l’échappement XML. Le texte décimal canonique supprime les zéros fractionnaires finaux. Une valeur nullable invalide renvoie None/null : omettez l’attribut au lieu d’écrire un attribut vide. Une valeur valide avec un zéro interne non initialisé renvoie "0". La lecture ne change jamais l’initialisation ni la validité ; le navigateur représente None de Rust par null.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-marshal-xml-attr" locale="fr" />

## Limites du navigateur

La source est limitée à 8192 octets UTF-8. Avant l’analyse numérique du cœur, Rust contrôle le premier scalaire YAML décodé ou le texte de l’élément XML, y compris les échappements, CDATA, la normalisation des traits de soulignement et l’insertion de zéros dans les flottants YAML. Les scalaires YAML acceptent au maximum 4096 caractères de chiffres hexadécimaux pour limiter l’allocation des entiers selon leur base ; les contrôles conservateurs peuvent rejeter des entrées acceptées nativement. L’échelle, le coefficient et le nombre d’opérations suivent la [politique de ressources](../resource-policy). Les handles déplacés ne peuvent plus être lus ni modifiés.

## Méthodes associées

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
