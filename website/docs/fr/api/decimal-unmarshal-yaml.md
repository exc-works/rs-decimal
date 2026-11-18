---
locale: fr
source_revision: 8604d7a6ce89905f3e2b134c8e0e9feabf234b540c25108481cbbaa1614462a5
translation_provenance: en-US-french-reviewed
api_id: "Decimal::unmarshal_yaml"
---
# Decimal::unmarshal_yaml

Décoder yaml dans une valeur existante.

```rust
fn unmarshal_yaml (& mut self , source : & str) -> Result < () , DecimalError >
```

## Paramètres, résultat et utilisation courante

source : &str/string ; modifie le récepteur et renvoie `Result<(), DecimalError>`/undefined. Rust exige yaml. Fournissez la source YAML d’origine pour conserver les guillemets, étiquettes et chiffres exacts accessibles à Rust. Le scalaire brut 012 est interprété en octal et vaut 10, tandis que "012" entre guillemets ou !!str 012 est analysé comme le décimal 12. Les étiquettes numériques, bases et traits de soulignement suivent le résolveur du cœur : 1_000 brut vaut 1000, mais "1_000" entre guillemets est invalide. Les booléens, collections et scalaires de bloc sont rejetés. Seul le premier document fournit la valeur ; les documents suivants doivent néanmoins avoir une syntaxe valide. Null ou un document vide laisse le récepteur existant inchangé, y compris son initialisation et son échelle. Un décodage réussi d’une entrée autre que null initialise le décimal et conserve l’échelle des valeurs numériquement non nulles. Les erreurs préservent tous les champs du récepteur et les catégories d’erreur de bibliothèque, dont Unmarshal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-unmarshal-yaml" locale="fr" />

## Limites du navigateur

La source est limitée à 8192 octets UTF-8. Avant l’analyse numérique du cœur, Rust contrôle le premier scalaire YAML décodé ou le texte de l’élément XML, y compris les échappements, CDATA, la normalisation des traits de soulignement et l’insertion de zéros dans les flottants YAML. Les scalaires YAML acceptent au maximum 4096 caractères de chiffres hexadécimaux pour limiter l’allocation des entiers selon leur base ; les contrôles conservateurs peuvent rejeter des entrées acceptées nativement. L’échelle, le coefficient et le nombre d’opérations suivent la [politique de ressources](../resource-policy). Les handles déplacés ne peuvent plus être lus ni modifiés.

## Méthodes associées

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
