---
locale: fr
source_revision: bb2070c90fbcf81b9f15485debb923fef5e838831e150577f2d283f059db73b6
translation_provenance: en-US-french-reviewed
api_id: Decimal::must_from_string
---
# Decimal::must_from_string

Analyser un texte décimal de confiance lorsque des données incorrectes constituent une erreur de programmation.

```rust
fn must_from_string (text : & str) -> Self
```

## Paramètres, résultat et remarques

text : &str/string ; renvoie Decimal. Une entrée valide produit le même état que new_from_string. Une syntaxe incorrecte déclenche un panic avec DecimalError en Rust ; le Wasm compilé déclenche un trap plutôt qu’une erreur de bibliothèque récupérable. Préférez new_from_string pour les entrées utilisateur. Un trap non intercepté termine l’exécution courante ; démarrez une nouvelle exécution avant de réutiliser Rust. Les contrôles de paramètres et de ressources rejettent toujours les entrées concernées avant l’appel à Rust.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-decimal-must-from-string" locale="fr" />

## Limites

Le bridge conserve les noms Rust et l’ordre des paramètres. Les fonctions libres sont directement disponibles dans le playground. Les entrées empruntées ne sont ni consommées ni modifiées ; le bridge rejette les handles invalides ou provenant d’une autre exécution. Toutes les opérations respectent la [politique de ressources](../resource-policy).

## Méthodes associées

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
