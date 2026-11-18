---
locale: fr
source_revision: 16b33be7c90300f9c4b236f60591e3b54c1078c3a213fb54df0f571ca8f60e09
translation_provenance: en-US-french-reviewed
api_id: Decimal::neg
---
# Decimal::neg

Créer un décimal indépendant de signe opposé.

```rust
fn neg (& self) -> Self
```

## Paramètres, résultat et remarques

Renvoie Decimal en conservant l’échelle et les zéros finaux. Zéro reste numériquement zéro ; le résultat est initialisé même pour une entrée par défaut.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-neg" locale="fr" />

## Erreurs et limites

Ces méthodes empruntent les entrées sans les consommer ni les modifier. L’opération de la bibliothèque ne renvoie pas Result. Le bridge rejette les handles libérés, déplacés ou provenant d’une autre exécution et applique la [politique de ressources](../resource-policy). Les exemples placent une sortie scalaire dans un champ `value` ; une sortie Decimal utilise son instantané complet.

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
