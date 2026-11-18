---
locale: fr
source_revision: 9a05736a78d12045af2704500028fdc4fdec0490c797ca6d8ba9130b66fbc153
translation_provenance: en-US-french-reviewed
api_id: min
---
# min

Copier la plus petite de deux valeurs décimales avec une fonction libre.

```rust
fn min (a : & Decimal , b : & Decimal) -> Decimal
```

## Paramètres, résultat et remarques

Emprunte a et b : &Decimal ; renvoie un Decimal indépendant. En cas d’égalité numérique, conserve l’échelle et l’état d’initialisation de a. La sémantique est celle de Decimal::min.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-min" locale="fr" />

## Limites

Le bridge conserve les noms Rust et l’ordre des paramètres. Les fonctions libres sont directement disponibles dans le playground. Les entrées empruntées ne sont ni consommées ni modifiées ; le bridge rejette les handles invalides ou provenant d’une autre exécution. Toutes les opérations respectent la [politique de ressources](../resource-policy).

## Méthodes associées

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
