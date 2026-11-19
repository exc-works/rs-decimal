---
locale: fr
source_revision: c4ab958e126151b59020a16b1317b5deb3282029f32339008d09669426a6a46f
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

## Méthodes associées

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
