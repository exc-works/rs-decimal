---
locale: fr
source_revision: 0d4a74db932ab126e7566fb630352abba8c528142a3d37c615a2694b7fac9c0a
translation_provenance: en-US-french-reviewed
api_id: max
---
# max

Copier la plus grande de deux valeurs décimales avec une fonction libre.

```rust
fn max (a : & Decimal , b : & Decimal) -> Decimal
```

## Paramètres, résultat et remarques

Emprunte a et b : &Decimal ; renvoie un Decimal indépendant. En cas d’égalité numérique, conserve l’échelle et l’état d’initialisation de a. La sémantique est celle de Decimal::max.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-max" locale="fr" />

## Limites

Le bridge conserve les noms Rust et l’ordre des paramètres. Les fonctions libres sont directement disponibles dans le playground. Les entrées empruntées ne sont ni consommées ni modifiées ; le bridge rejette les handles invalides ou provenant d’une autre exécution. Toutes les opérations respectent la [politique de ressources](../resource-policy).

## Méthodes associées

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
