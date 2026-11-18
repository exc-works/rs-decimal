---
locale: fr
source_revision: b70b6d632eef5a013491ea47ed27e9d8eea36fee70b75cd0db8637b8b6a6ccc5
translation_provenance: en-US-french-reviewed
api_id: Decimal::coefficient
---
# Decimal::coefficient

Inspecter le coefficient entier exact non mis à l’échelle.

```rust
fn coefficient (& self) -> & BigInt
```

## Paramètres, résultat et remarques

Rust emprunte &BigInt ; le bridge renvoie un instantané bigint immuable, pas une référence reflétant les modifications. Le coefficient inclut les zéros finaux et n’est pas la partie entière du décimal. L’exemple affiche bigint sous forme de texte pour conserver tous les chiffres dans JSON.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-coefficient" locale="fr" />

## Erreurs et limites

Ces méthodes empruntent les entrées sans les consommer ni les modifier. L’opération de la bibliothèque ne renvoie pas Result. Le bridge rejette les handles libérés, déplacés ou provenant d’une autre exécution et applique la [politique de ressources](../resource-policy). Les exemples placent une sortie scalaire dans un champ `value` ; une sortie Decimal utilise son instantané complet.

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
