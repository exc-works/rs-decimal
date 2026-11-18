---
locale: fr
source_revision: 1cd08b3844b9339373458bc008707757ca17e7772da4c888bc88a31f9efef788
translation_provenance: en-US-french-reviewed
api_id: Decimal::min
---
# Decimal::min

Copier la plus petite de deux valeurs numériques.

```rust
fn min (& self , other : & Self) -> Self
```

## Paramètres, résultat et remarques

Emprunte other : &Decimal ; renvoie un Decimal indépendant. En cas d’égalité, conserve l’échelle et l’état d’initialisation du premier opérande. En Rust, utilisez Decimal::min(&left, &right) pour éviter la méthode du trait Ord::min qui consomme la valeur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-min" locale="fr" />

## Erreurs et limites

Ces méthodes empruntent les entrées sans les consommer ni les modifier. L’opération de la bibliothèque ne renvoie pas Result. Le bridge rejette les handles libérés, déplacés ou provenant d’une autre exécution et applique la [politique de ressources](../resource-policy). Les exemples placent une sortie scalaire dans un champ `value` ; une sortie Decimal utilise son instantané complet.

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
