---
locale: fr
source_revision: 404c61af98ea51d383b516ba1e691c0c718c44923614dbeaca2d2a7efd410ffc
translation_provenance: en-US-french-reviewed
api_id: "Decimal::unmarshal_binary"
---
# Decimal::unmarshal_binary

Remplacer un décimal à partir d’un scalaire binaire compatible Go.

```rust
fn unmarshal_binary (& mut self , bytes : & [u8]) -> Result < () , DecimalError >
```

## Paramètres, résultat et utilisation courante

bytes : &[u8]/Uint8Array ; modifie le récepteur et renvoie `Result<(), DecimalError>`/undefined. Les quatre premiers octets encodent l’échelle gros-boutiste, suivie de l’en-tête Gob 2 (positif) ou 3 (négatif) puis de la valeur absolue. Un décodage réussi conserve l’échelle encodée. Une entrée vide crée un zéro initialisé, contrairement au zéro par défaut non initialisé. Un préfixe de quatre octets sans charge utile Gob est aussi accepté comme zéro initialisé à cette échelle. Les préfixes trop courts, versions incorrectes et échelles supérieures à 131072 renvoient des erreurs Unmarshal du cœur. Les erreurs préservent le récepteur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-unmarshal-binary" locale="fr" />

## Limites du navigateur

L’entrée doit être un Uint8Array et kind un entier de 0 à 255. Les entrées sont limitées à 65536 octets avant copie dans Wasm. Rust vérifie une échelle binaire maximale de 1000 et une taille réelle du coefficient maximale de 16384 bits. Les chaînes BSON suivent la limite de texte de 8192 octets ; l’expansion de l’exposant Decimal128 est aussi soumise au [contrôle des ressources](../resource-policy). Ces limites peuvent être plus strictes que celles de la bibliothèque native. Les contrôles précèdent la modification du récepteur ; les erreurs du cœur et de ressources restent distinctes, et les handles déplacés ne peuvent pas être réutilisés.

## Méthodes associées

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
