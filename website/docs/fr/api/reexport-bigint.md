---
locale: fr
source_revision: f07542c46478fd9edd09bbf3af9bd03d15608eb33e9ed99beb51b022df098d6a
translation_provenance: en-US-french-reviewed
api_id: reexport::BigInt
---
# reexport::BigInt

Importer BigInt depuis la racine du crate.

```rust
pub use num_bigint::BigInt as BigInt
```

## Paramètres, résultat et utilisation courante

Il s’agit de la réexportation publique Rust, pas d’un nouveau type ou d’une nouvelle implémentation. La syntaxe d’import est testée nativement. Les équivalents dans le navigateur sont respectivement bigint, un objet de champs rationnels exacts et la fonction globale most_significant_bit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-reexport-bigint" locale="fr" />

## Méthodes associées

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
