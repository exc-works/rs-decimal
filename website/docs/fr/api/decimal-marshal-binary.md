---
locale: fr
source_revision: aa67b8e6e6f3fb7b6a66f11573d9907fb9dc057abd9547e4d0c0474969528084
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

## Méthodes associées

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
