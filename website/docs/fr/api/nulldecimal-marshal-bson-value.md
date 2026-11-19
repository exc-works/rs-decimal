---
locale: fr
source_revision: 9fa4dc63685c8b7863c54238c3b1bc5377c717d45438360151595cb826deb45d
translation_provenance: en-US-french-reviewed
api_id: "NullDecimal::marshal_bson_value"
---
# NullDecimal::marshal_bson_value

Encoder un octet de type BSON et sa charge utile scalaire.

```rust
fn marshal_bson_value (& self) -> Result < (u8 , Vec < u8 >) , DecimalError >
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie `Result<(u8, Vec<u8>), DecimalError>`/[number, Uint8Array]. Rust exige la fonctionnalité bson, activée dans le build du navigateur. Le tuple contient un scalaire BSON brut, pas un document BSON complet. Les valeurs nullables valides sont encodées en chaînes BSON, y compris celles dont le zéro interne n’est pas initialisé. Les valeurs invalides sont encodées en type 10 avec une charge utile vide. La charge utile de chaîne commence par une longueur en octets i32 petit-boutiste incluant le NUL final, puis contient le texte décimal canonique sans zéros fractionnaires finaux. Les octets renvoyés sont une copie indépendante ; l’encodage ne change ni l’initialisation ni la validité.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-nulldecimal-marshal-bson-value" locale="fr" />

## Méthodes associées

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
