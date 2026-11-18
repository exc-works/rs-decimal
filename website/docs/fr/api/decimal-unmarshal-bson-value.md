---
locale: fr
source_revision: d77e5dd32cf21e2ddfd9d7df9654834b1090bc84b953f82d973c756dd33fd206
translation_provenance: en-US-french-reviewed
api_id: "Decimal::unmarshal_bson_value"
---
# Decimal::unmarshal_bson_value

Décoder un scalaire BSON dans une valeur existante.

```rust
fn unmarshal_bson_value (& mut self , kind : u8 , data : & [u8]) -> Result < () , DecimalError >
```

## Paramètres, résultat et utilisation courante

kind : u8/number, data : &[u8]/Uint8Array ; modifie le récepteur et renvoie `Result<(), DecimalError>`/undefined. Rust exige bson. Prend en charge double (1), string (2), null (10), int32 (16), int64 (18) et Decimal128 (19) ; les charges utiles numériques utilisent les octets petit-boutistes de BSON. Fournissez les octets bruts pour int64 afin d’éviter la perte de précision d’un number JavaScript. BSON null réinitialise le récepteur à un zéro non initialisé, contrairement à unmarshal_json(null), qui le laisse inchangé. Un décodage nullable réussi d’une entrée autre que null définit valid=true. Les types non pris en charge, données tronquées et nombres non finis échouent sans modification partielle. Détails de compatibilité : le décodage de chaîne ignore son dernier octet déclaré sans vérifier NUL ; les octets au-delà de la charge utile requise sont ignorés. Decimal128 est décodé par le cœur Rust et conserve la précision décimale prise en charge ; la sortie de marshal_bson_value reste une chaîne.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-unmarshal-bson-value" locale="fr" />

## Limites du navigateur

L’entrée doit être un Uint8Array et kind un entier de 0 à 255. Les entrées sont limitées à 65536 octets avant copie dans Wasm. Rust vérifie une échelle binaire maximale de 1000 et une taille réelle du coefficient maximale de 16384 bits. Les chaînes BSON suivent la limite de texte de 8192 octets ; l’expansion de l’exposant Decimal128 est aussi soumise au [contrôle des ressources](../resource-policy). Ces limites peuvent être plus strictes que celles de la bibliothèque native. Les contrôles précèdent la modification du récepteur ; les erreurs du cœur et de ressources restent distinctes, et les handles déplacés ne peuvent pas être réutilisés.

## Méthodes associées

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
