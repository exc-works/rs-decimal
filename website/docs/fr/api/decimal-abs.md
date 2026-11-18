---
locale: fr
source_revision: aa32a5479f7baf77d635cc3726288c9b7c2cfa292167bcd9b91265a058234a8b
translation_provenance: en-US-french-reviewed
api_id: Decimal::abs
---
# Decimal::abs

Créer un décimal indépendant contenant la valeur absolue.

```rust
fn abs (& self) -> Self
```

## Paramètres, résultat et remarques

Renvoie Decimal en conservant l’échelle et les zéros finaux. Le résultat est initialisé même si l’entrée est la valeur par défaut ; l’entrée reste inchangée.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-abs" locale="fr" />

## Erreurs et limites

Ces méthodes empruntent les entrées sans les consommer ni les modifier. L’opération de la bibliothèque ne renvoie pas Result. Le bridge rejette les handles libérés, déplacés ou provenant d’une autre exécution et applique la [politique de ressources](../resource-policy). Les exemples placent une sortie scalaire dans un champ `value` ; une sortie Decimal utilise son instantané complet.

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
