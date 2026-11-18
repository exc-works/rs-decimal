---
locale: fr
source_revision: 72730d41ab3e71fa242fbf48a88a0c6a502506e0eb80815d9391890ebfff87d8
translation_provenance: en-US-french-reviewed
api_id: "NullDecimal::unmarshal_xml_attr"
---
# NullDecimal::unmarshal_xml_attr

Décoder xml_attr dans une valeur existante.

```rust
fn unmarshal_xml_attr (& mut self , value : & str) -> Result < () , DecimalError >
```

## Paramètres, résultat et utilisation courante

value : &str/string ; modifie le récepteur et renvoie `Result<(), DecimalError>`/undefined. Rust exige xml. Fournissez la valeur d’attribut dont les entités sont déjà décodées, pas un fragment d’attribut XML : "1.23" est valide, mais `"&#49;.23"` n’est pas décodé à nouveau. Supprime les espaces aux extrémités et analyse le texte décimal exact. Un contenu vide efface valid et réinitialise le Decimal interne à un zéro non initialisé. Un décodage réussi d’une entrée autre que null initialise le décimal et conserve l’échelle des valeurs numériquement non nulles. Définit valid=true. Les erreurs préservent tous les champs du récepteur et les catégories d’erreur de bibliothèque, dont Unmarshal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-unmarshal-xml-attr" locale="fr" />

## Limites du navigateur

La source est limitée à 8192 octets UTF-8. Avant l’analyse numérique du cœur, Rust contrôle le premier scalaire YAML décodé ou le texte de l’élément XML, y compris les échappements, CDATA, la normalisation des traits de soulignement et l’insertion de zéros dans les flottants YAML. Les scalaires YAML acceptent au maximum 4096 caractères de chiffres hexadécimaux pour limiter l’allocation des entiers selon leur base ; les contrôles conservateurs peuvent rejeter des entrées acceptées nativement. L’échelle, le coefficient et le nombre d’opérations suivent la [politique de ressources](../resource-policy). Les handles déplacés ne peuvent plus être lus ni modifiés.

## Méthodes associées

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
