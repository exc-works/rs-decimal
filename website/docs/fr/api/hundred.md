---
locale: fr
source_revision: 075546f79884d636c4e552add1629511a5f2dca740186a5700262f084c5bd8a5
translation_provenance: en-US-french-reviewed
api_id: hundred
---
# hundred

Construire l’entier décimal cent.

```rust
fn hundred () -> Decimal
```

## Paramètres, résultat et remarques

Sans paramètre ; renvoie un nouveau Decimal initialisé d’échelle zéro. C’est un constructeur de valeur, pas une constante mutable partagée.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-hundred" locale="fr" />

## Limites

Le bridge conserve les noms Rust et l’ordre des paramètres. Les fonctions libres sont directement disponibles dans le playground. Les entrées empruntées ne sont ni consommées ni modifiées ; le bridge rejette les handles invalides ou provenant d’une autre exécution. Toutes les opérations respectent la [politique de ressources](../resource-policy).

## Méthodes associées

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
