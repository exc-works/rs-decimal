---
locale: fr
source_revision: 1933adb169124e0ed9d429b21c95687e4fd59240647c4b0f684ca7bc76aabd2b
translation_provenance: en-US-french-reviewed
api_id: Decimal::big_int
---
# Decimal::big_int

Copier le coefficient entier exact non mis à l’échelle.

```rust
fn big_int (& self) -> BigInt
```

## Paramètres, résultat et remarques

Rust renvoie un BigInt indépendant ; le bridge renvoie bigint. Les zéros finaux sont inclus, contrairement à une conversion entière par troncature. L’exemple affiche bigint sous forme de texte pour JSON.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-big-int" locale="fr" />

## Erreurs et limites

Ces méthodes empruntent les entrées sans les consommer ni les modifier. L’opération de la bibliothèque ne renvoie pas Result. Le bridge rejette les handles libérés, déplacés ou provenant d’une autre exécution et applique la [politique de ressources](../resource-policy). Les exemples placent une sortie scalaire dans un champ `value` ; une sortie Decimal utilise son instantané complet.

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
