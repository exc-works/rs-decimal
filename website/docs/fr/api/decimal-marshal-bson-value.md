---
locale: fr
source_revision: 44afbeff0d59cbde33e83e7f33c7e22c9903950f7ecf43d6f8230c6cde4b2ec4
translation_provenance: en-US-french-reviewed
api_id: "Decimal::marshal_bson_value"
---
# Decimal::marshal_bson_value

Encoder un octet de type BSON et sa charge utile scalaire.

```rust
fn marshal_bson_value (& self) -> Result < (u8 , Vec < u8 >) , DecimalError >
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie `Result<(u8, Vec<u8>), DecimalError>`/[number, Uint8Array]. Rust exige la fonctionnalité bson, activée dans le build du navigateur. Le tuple contient un scalaire BSON brut, pas un document BSON complet. Les valeurs initialisées sont encodées en chaînes BSON (type 2), pas en Decimal128. Un zéro non initialisé est encodé en type 10 avec une charge utile vide. La charge utile de chaîne commence par une longueur en octets i32 petit-boutiste incluant le NUL final, puis contient le texte décimal canonique sans zéros fractionnaires finaux. Les octets renvoyés sont une copie indépendante ; l’encodage ne change ni l’initialisation ni la validité.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-marshal-bson-value" locale="fr" />

## Méthodes associées

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
