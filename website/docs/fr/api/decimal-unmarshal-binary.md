---
locale: fr
source_revision: 8a589312845f2d23d85f3d6da1987ee87f79dfecdc97a705386e9a336b17d7ed
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

## Méthodes associées

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
