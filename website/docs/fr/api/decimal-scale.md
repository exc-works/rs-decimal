---
locale: fr
source_revision: c63b4582e5ea4e7eac387ce70eb29aed40ab3d4cb5d5c76ae95a6cd8fa1f8f69
translation_provenance: en-US-french-reviewed
api_id: Decimal::scale
---
# Decimal::scale

Lire l’échelle décimale non négative stockée.

```rust
fn scale (& self) -> usize
```

## Paramètres, résultat et remarques

Renvoie usize sous forme d’un number exact dans la limite d’échelle du playground. La valeur est identique à precision().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-scale" locale="fr" />

## Erreurs et limites

Ces méthodes empruntent les entrées sans les consommer ni les modifier. L’opération de la bibliothèque ne renvoie pas Result. Le bridge rejette les handles libérés, déplacés ou provenant d’une autre exécution et applique la [politique de ressources](../resource-policy). Les exemples placent une sortie scalaire dans un champ `value` ; une sortie Decimal utilise son instantané complet.

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
