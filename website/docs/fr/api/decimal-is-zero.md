---
locale: fr
source_revision: aa764a04d58bbac840fe64bca919cb70175f064a9010f9a840a1f56ffacabc1c
translation_provenance: en-US-french-reviewed
api_id: Decimal::is_zero
---
# Decimal::is_zero

Tester le zéro numérique avant une opération dépendant de la valeur.

```rust
fn is_zero (& self) -> bool
```

## Paramètres, résultat et remarques

Renvoie bool/boolean, et également true pour la valeur par défaut non initialisée. Utilisez is_nil() si l’état d’initialisation importe.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-zero" locale="fr" />

## Erreurs et limites

Ces méthodes empruntent les entrées sans les consommer ni les modifier. L’opération de la bibliothèque ne renvoie pas Result. Le bridge rejette les handles libérés, déplacés ou provenant d’une autre exécution et applique la [politique de ressources](../resource-policy). Les exemples placent une sortie scalaire dans un champ `value` ; une sortie Decimal utilise son instantané complet.

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
