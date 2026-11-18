---
locale: fr
source_revision: 40283a0b4501fabd3b307e9ac7d1a0dd292a797503fe33899ba506e5b43fdac8
translation_provenance: en-US-french-reviewed
api_id: "reexport::PRECISION_FIXED_SIZE"
---
# reexport::PRECISION_FIXED_SIZE

Lire la longueur en octets du préfixe binaire de précision.

```rust
pub use binary::PRECISION_FIXED_SIZE as PRECISION_FIXED_SIZE
```

## Paramètres, résultat et utilisation courante

Constante Rust usize de valeur 4 : longueur en octets du préfixe d’échelle gros-boutiste dans le format binaire compatible Go. Ce n’est ni un nombre de chiffres décimaux ni une limite de précision. Le navigateur expose un number en lecture seule provenant de Rust. La réexportation à la racine du crate relève de la syntaxe d’import Rust.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-reexport-precision-fixed-size" locale="fr" />

## Limites du navigateur

L’entrée doit être un Uint8Array et kind un entier de 0 à 255. Les entrées sont limitées à 65536 octets avant copie dans Wasm. Rust vérifie une échelle binaire maximale de 1000 et une taille réelle du coefficient maximale de 16384 bits. Les chaînes BSON suivent la limite de texte de 8192 octets ; l’expansion de l’exposant Decimal128 est aussi soumise au [contrôle des ressources](../resource-policy). Ces limites peuvent être plus strictes que celles de la bibliothèque native. Les contrôles précèdent la modification du récepteur ; les erreurs du cœur et de ressources restent distinctes, et les handles déplacés ne peuvent pas être réutilisés.

## Méthodes associées

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
