---
locale: fr
source_revision: c8a99895a920cbd49f4dba891779055021c167a0addb6791872293807a1937f8
translation_provenance: en-US-french-reviewed
api_id: Decimal::precision
---
# Decimal::precision

Lire le nombre de décimales stockées dans une valeur.

```rust
fn precision (& self) -> isize
```

## Paramètres, résultat et remarques

Renvoie un isize (un number entier dans le navigateur). Il s’agit de l’échelle, pas du nombre de chiffres significatifs. Les zéros finaux sont comptés.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-precision" locale="fr" />

## Erreurs et limites

Ces méthodes empruntent les entrées sans les consommer ni les modifier. L’opération de la bibliothèque ne renvoie pas Result. Le bridge rejette les handles libérés, déplacés ou provenant d’une autre exécution et applique la [politique de ressources](../resource-policy). Les exemples placent une sortie scalaire dans un champ `value` ; une sortie Decimal utilise son instantané complet.

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
