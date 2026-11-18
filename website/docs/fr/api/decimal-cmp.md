---
locale: fr
source_revision: 19a39638a4734aeba49de6f5af4a9f79e89b842f7b78ba2cf7aeb511f0ebcd6c
translation_provenance: en-US-french-reviewed
api_id: Decimal::cmp
---
# Decimal::cmp

Comparer deux valeurs numériques avec un résultat à trois états.

```rust
fn cmp (& self , other : & Self) -> i32
```

## Paramètres, résultat et remarques

Emprunte other : &Decimal (Decimal en JavaScript). Renvoie i32/number : -1, 0 ou 1. Des échelles et états d’initialisation différents ne modifient pas l’égalité numérique. Il s’agit de la méthode inhérente, et non de Ord::cmp qui renvoie Ordering.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-cmp" locale="fr" />

## Erreurs et limites

Ces méthodes empruntent les entrées sans les consommer ni les modifier. L’opération de la bibliothèque ne renvoie pas Result. Le bridge rejette les handles libérés, déplacés ou provenant d’une autre exécution et applique la [politique de ressources](../resource-policy). Les exemples placent une sortie scalaire dans un champ `value` ; une sortie Decimal utilise son instantané complet.

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
