---
locale: fr
source_revision: 60ca5295cf77ea0e6cccdadb5d937c5adbdd69051a5dbac42387fec9ee90828c
translation_provenance: en-US-french-reviewed
api_id: Decimal::lte
---
# Decimal::lte

Déterminer si cette valeur est inférieure ou égale à une autre.

```rust
fn lte (& self , other : & Self) -> bool
```

## Paramètres, résultat et remarques

Emprunte other : &Decimal ; renvoie bool/boolean. L’égalité est numérique, indépendamment de l’échelle et de l’état d’initialisation.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-lte" locale="fr" />

## Erreurs et limites

Ces méthodes empruntent les entrées sans les consommer ni les modifier. L’opération de la bibliothèque ne renvoie pas Result. Le bridge rejette les handles libérés, déplacés ou provenant d’une autre exécution et applique la [politique de ressources](../resource-policy). Les exemples placent une sortie scalaire dans un champ `value` ; une sortie Decimal utilise son instantané complet.

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
