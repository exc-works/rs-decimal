---
locale: fr
source_revision: fe8ae2df48c06b20c3ac1c7bbd224d12734d4193d035a1a96ed7e524e49fd64b
translation_provenance: en-US-french-reviewed
api_id: Decimal::lt
---
# Decimal::lt

Déterminer si cette valeur est inférieure à une autre.

```rust
fn lt (& self , other : & Self) -> bool
```

## Paramètres, résultat et remarques

Emprunte other : &Decimal ; renvoie bool/boolean. Des valeurs numériquement égales donnent false, même si leurs échelles stockées diffèrent.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-lt" locale="fr" />

## Erreurs et limites

Ces méthodes empruntent les entrées sans les consommer ni les modifier. L’opération de la bibliothèque ne renvoie pas Result. Le bridge rejette les handles libérés, déplacés ou provenant d’une autre exécution et applique la [politique de ressources](../resource-policy). Les exemples placent une sortie scalaire dans un champ `value` ; une sortie Decimal utilise son instantané complet.

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
