---
locale: fr
source_revision: 775716427f3737c65dee2b46317125f81d53dce3779b9de7255849e290c2308b
translation_provenance: en-US-french-reviewed
api_id: Decimal::is_positive
---
# Decimal::is_positive

Déterminer si la valeur numérique est strictement positive.

```rust
fn is_positive (& self) -> bool
```

## Paramètres, résultat et remarques

Renvoie bool/boolean. Zéro, y compris la valeur par défaut non initialisée, donne false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-positive" locale="fr" />

## Erreurs et limites

Ces méthodes empruntent les entrées sans les consommer ni les modifier. L’opération de la bibliothèque ne renvoie pas Result. Le bridge rejette les handles libérés, déplacés ou provenant d’une autre exécution et applique la [politique de ressources](../resource-policy). Les exemples placent une sortie scalaire dans un champ `value` ; une sortie Decimal utilise son instantané complet.

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
