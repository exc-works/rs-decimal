---
locale: fr
source_revision: 50a6a7b47a0234b339cf6d517b9928ec7d1775457e7eacbe40da86286996212a
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

## Méthodes associées

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
