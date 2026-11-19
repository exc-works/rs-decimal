---
locale: fr
source_revision: 65435d6f54d8777702cf2f629258b85e312b71241d8a0725f7c209d6e7b4a200
translation_provenance: en-US-french-reviewed
api_id: "PRECISION_FIXED_SIZE"
---
# PRECISION_FIXED_SIZE

Lire la longueur en octets du préfixe binaire de précision.

```rust
const PRECISION_FIXED_SIZE: usize = 4
```

## Paramètres, résultat et utilisation courante

Constante Rust usize de valeur 4 : longueur en octets du préfixe d’échelle gros-boutiste dans le format binaire compatible Go. Ce n’est ni un nombre de chiffres décimaux ni une limite de précision. Le navigateur expose un number en lecture seule provenant de Rust. La réexportation à la racine du crate relève de la syntaxe d’import Rust.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-precision-fixed-size" locale="fr" />

## Méthodes associées

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
