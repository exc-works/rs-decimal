---
locale: fr
source_revision: 59688cddd572412f36dd0f36cc15c34efbc891dab38cd57b614763347d469369
translation_provenance: en-US-french-reviewed
api_id: Decimal::is_nil
---
# Decimal::is_nil

Déterminer si un décimal est la valeur par défaut non initialisée.

```rust
fn is_nil (& self) -> bool
```

## Paramètres, résultat et remarques

Renvoie bool/boolean. La valeur par défaut n’est pas initialisée, tandis qu’un zéro construit l’est ; les deux sont numériquement égaux. Ce n’est pas l’indicateur de validité de NullDecimal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-nil" locale="fr" />

## Erreurs et limites

Ces méthodes empruntent les entrées sans les consommer ni les modifier. L’opération de la bibliothèque ne renvoie pas Result. Le bridge rejette les handles libérés, déplacés ou provenant d’une autre exécution et applique la [politique de ressources](../resource-policy). Les exemples placent une sortie scalaire dans un champ `value` ; une sortie Decimal utilise son instantané complet.

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
