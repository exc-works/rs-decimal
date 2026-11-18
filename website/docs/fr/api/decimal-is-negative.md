---
locale: fr
source_revision: 9ffad200b8eee0be119e53ec6189311fb6b54cada31774d04149bdc2b868ac4e
translation_provenance: en-US-french-reviewed
api_id: Decimal::is_negative
---
# Decimal::is_negative

Déterminer si la valeur numérique est strictement négative.

```rust
fn is_negative (& self) -> bool
```

## Paramètres, résultat et remarques

Renvoie bool/boolean. Zéro, y compris la valeur par défaut non initialisée, donne false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-negative" locale="fr" />

## Erreurs et limites

Ces méthodes empruntent les entrées sans les consommer ni les modifier. L’opération de la bibliothèque ne renvoie pas Result. Le bridge rejette les handles libérés, déplacés ou provenant d’une autre exécution et applique la [politique de ressources](../resource-policy). Les exemples placent une sortie scalaire dans un champ `value` ; une sortie Decimal utilise son instantané complet.

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
