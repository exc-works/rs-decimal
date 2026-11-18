---
locale: fr
source_revision: 18a49744a56694725a1fe7d677be386e97a4c76f0669871a289c86113d20f3e2
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

## Limites du navigateur

L’entrée doit être un Uint8Array et kind un entier de 0 à 255. Les entrées sont limitées à 65536 octets avant copie dans Wasm. Rust vérifie une échelle binaire maximale de 1000 et une taille réelle du coefficient maximale de 16384 bits. Les chaînes BSON suivent la limite de texte de 8192 octets ; l’expansion de l’exposant Decimal128 est aussi soumise au [contrôle des ressources](../resource-policy). Ces limites peuvent être plus strictes que celles de la bibliothèque native. Les contrôles précèdent la modification du récepteur ; les erreurs du cœur et de ressources restent distinctes, et les handles déplacés ne peuvent pas être réutilisés.

## Méthodes associées

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
