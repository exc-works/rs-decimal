---
locale: fr
source_revision: 5956431328357316a87f05c2d1543bb100b12923bb2d50b1e21ff3f96212db45
translation_provenance: en-US-french-reviewed
api_id: Decimal::max
---
# Decimal::max

Copier la plus grande de deux valeurs numériques.

```rust
fn max (& self , other : & Self) -> Self
```

## Paramètres, résultat et remarques

Emprunte other : &Decimal ; renvoie un Decimal indépendant. En cas d’égalité, conserve l’échelle et l’état d’initialisation du premier opérande. En Rust, utilisez Decimal::max(&left, &right) pour sélectionner explicitement cette méthode inhérente avec emprunt ; Ord::max prend possession de la valeur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-max" locale="fr" />

## Erreurs et limites

Ces méthodes empruntent les entrées sans les consommer ni les modifier. L’opération de la bibliothèque ne renvoie pas Result. Le bridge rejette les handles libérés, déplacés ou provenant d’une autre exécution et applique la [politique de ressources](../resource-policy). Les exemples placent une sortie scalaire dans un champ `value` ; une sortie Decimal utilise son instantané complet.

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
