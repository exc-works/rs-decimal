---
locale: fr
source_revision: 939b86ba367c0b35cb69b9f2664eb74fafe4279f8b1b81c20315cc4e07b18382
translation_provenance: en-US-french-reviewed
api_id: one
---
# one

Construire l’entier décimal un.

```rust
fn one () -> Decimal
```

## Paramètres, résultat et remarques

Sans paramètre ; renvoie un nouveau Decimal initialisé d’échelle zéro. Modifier ou libérer un résultat n’affecte pas les appels suivants.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-one" locale="fr" />

## Limites

Le bridge conserve les noms Rust et l’ordre des paramètres. Les fonctions libres sont directement disponibles dans le playground. Les entrées empruntées ne sont ni consommées ni modifiées ; le bridge rejette les handles invalides ou provenant d’une autre exécution. Toutes les opérations respectent la [politique de ressources](../resource-policy).

## Méthodes associées

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
