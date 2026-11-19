---
locale: fr
source_revision: f67e1caa0fb769c278b4d8058441646a0cf632963d9da4fd8b19596b64289dc8
translation_provenance: en-US-french-reviewed
api_id: reexport::BigRational
---
# reexport::BigRational

Importer BigRational depuis la racine du crate.

```rust
pub use num_rational::BigRational as BigRational
```

## Paramètres, résultat et utilisation courante

Il s’agit de la réexportation publique Rust, pas d’un nouveau type ou d’une nouvelle implémentation. La syntaxe d’import est testée nativement. Les équivalents dans le navigateur sont respectivement bigint, un objet de champs rationnels exacts et la fonction globale most_significant_bit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-reexport-bigrational" locale="fr" />

## Méthodes associées

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
