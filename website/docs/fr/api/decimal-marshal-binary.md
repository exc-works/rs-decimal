---
locale: fr
source_revision: b225904d6e477d16ac1df6ba10bcfee3249227a343581851ba1cc46cc84f5863
translation_provenance: en-US-french-reviewed
api_id: "Decimal::marshal_binary"
---
# Decimal::marshal_binary

Encoder le format de transmission precision/Gob compatible Go.

```rust
fn marshal_binary (& self) -> Result < Vec < u8 > , DecimalError >
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie `Result<Vec<u8>, DecimalError>`/Uint8Array. Les valeurs initialisées perdent leurs zéros fractionnaires finaux avant l’écriture d’une échelle gros-boutiste sur quatre octets, suivie de l’octet version/signe big.Int Gob puis de la valeur absolue gros-boutiste. C’est une charge utile scalaire décimale, pas un flottant IEEE. Le zéro non initialisé est encodé en octets vides ; le zéro initialisé en [0,0,0,0,2]. La copie renvoyée peut être modifiée sans changer le Decimal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-marshal-binary" locale="fr" />

## Limites du navigateur

L’entrée doit être un Uint8Array et kind un entier de 0 à 255. Les entrées sont limitées à 65536 octets avant copie dans Wasm. Rust vérifie une échelle binaire maximale de 1000 et une taille réelle du coefficient maximale de 16384 bits. Les chaînes BSON suivent la limite de texte de 8192 octets ; l’expansion de l’exposant Decimal128 est aussi soumise au [contrôle des ressources](../resource-policy). Ces limites peuvent être plus strictes que celles de la bibliothèque native. Les contrôles précèdent la modification du récepteur ; les erreurs du cœur et de ressources restent distinctes, et les handles déplacés ne peuvent pas être réutilisés.

## Méthodes associées

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
