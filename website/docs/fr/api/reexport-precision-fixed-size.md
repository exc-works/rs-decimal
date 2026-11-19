---
locale: fr
source_revision: 8de700fe2bea77817d4cc64ce868f77fd5301e630577d698cc1cf5a3b4f76ce5
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

## Méthodes associées

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
