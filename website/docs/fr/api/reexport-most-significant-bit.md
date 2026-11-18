---
locale: fr
source_revision: fe0a2cbd7e86aa455f6274096dc78e80786ab081e8fd78826b22f34970a7dfa9
translation_provenance: en-US-french-reviewed
api_id: reexport::most_significant_bit
---
# reexport::most_significant_bit

Importer most_significant_bit depuis la racine du crate.

```rust
pub use conversion::most_significant_bit as most_significant_bit
```

## Paramètres, résultat et utilisation courante

Il s’agit de la réexportation publique Rust, pas d’un nouveau type ou d’une nouvelle implémentation. La syntaxe d’import est testée nativement. Les équivalents dans le navigateur sont respectivement bigint, un objet de champs rationnels exacts et la fonction globale most_significant_bit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-reexport-most-significant-bit" locale="fr" />

## Limites du navigateur

Le transport des entiers ne passe pas par des number JavaScript avec perte. Les tuples de conversion conservent l’ordre Rust valeur/indicateur ; false ne garantit pas un résultat exact. Les entrées empruntées restent inchangées et les constructeurs initialisent les résultats. La [politique de ressources](../resource-policy) limite les entrées entières, l’échelle de sortie et la croissance des coefficients rationnels, séparément des erreurs de bibliothèque. La syntaxe d’import Rust est explicitement indiquée comme testée nativement.

## Méthodes associées

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
