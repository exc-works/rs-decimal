---
locale: fr
source_revision: aaa15879504e10287ba605474fa0c07c65d1d6607caa2a6ac35b407fe6cd46e4
translation_provenance: en-US-french-reviewed
api_id: Decimal::is_not_zero
---
# Decimal::is_not_zero

Déterminer si la valeur numérique est différente de zéro.

```rust
fn is_not_zero (& self) -> bool
```

## Paramètres, résultat et remarques

Renvoie bool/boolean, l’inverse de is_zero(). La valeur par défaut non initialisée donne false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-not-zero" locale="fr" />

## Erreurs et limites

Ces méthodes empruntent les entrées sans les consommer ni les modifier. L’opération de la bibliothèque ne renvoie pas Result. Le bridge rejette les handles libérés, déplacés ou provenant d’une autre exécution et applique la [politique de ressources](../resource-policy). Les exemples placent une sortie scalaire dans un champ `value` ; une sortie Decimal utilise son instantané complet.

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
