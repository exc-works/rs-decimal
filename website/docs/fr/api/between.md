---
locale: fr
source_revision: 4488da41f235921047e8f85b8a68b47bc497bf199529326773333d8f22d38585
translation_provenance: en-US-french-reviewed
api_id: between
---
# between

Déterminer si un décimal appartient à un intervalle numérique fermé.

```rust
fn between (v : & Decimal , lower : & Decimal , upper : & Decimal) -> bool
```

## Paramètres, résultat et remarques

Emprunte v, lower et upper : &Decimal ; renvoie bool/boolean pour lower <= v <= upper. L’égalité ignore la représentation. Les bornes ne sont pas réordonnées : lower > upper produit false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-between" locale="fr" />

## Limites

Le bridge conserve les noms Rust et l’ordre des paramètres. Les fonctions libres sont directement disponibles dans le playground. Les entrées empruntées ne sont ni consommées ni modifiées ; le bridge rejette les handles invalides ou provenant d’une autre exécution. Toutes les opérations respectent la [politique de ressources](../resource-policy).

## Méthodes associées

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
