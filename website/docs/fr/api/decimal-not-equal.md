---
locale: fr
source_revision: 7f94821cf570d2860956d25bb69a5d1cdbe5a253fac837b02903b87236f85c19
translation_provenance: en-US-french-reviewed
api_id: Decimal::not_equal
---
# Decimal::not_equal

Déterminer si deux décimaux représentent des nombres différents.

```rust
fn not_equal (& self , other : & Self) -> bool
```

## Paramètres, résultat et remarques

Emprunte other : &Decimal ; renvoie bool/boolean. C’est l’inverse de equal(), et non une comparaison des instantanés du coefficient et de l’échelle.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-not-equal" locale="fr" />

## Erreurs et limites

Ces méthodes empruntent les entrées sans les consommer ni les modifier. L’opération de la bibliothèque ne renvoie pas Result. Le bridge rejette les handles libérés, déplacés ou provenant d’une autre exécution et applique la [politique de ressources](../resource-policy). Les exemples placent une sortie scalaire dans un champ `value` ; une sortie Decimal utilise son instantané complet.

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
