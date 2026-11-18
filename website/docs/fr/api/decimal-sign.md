---
locale: fr
source_revision: 828ccfc55974951d7ced64d269a1959b7a45efe557a7f4202f79dcfc3632ea3f
translation_provenance: en-US-french-reviewed
api_id: Decimal::sign
---
# Decimal::sign

Lire le signe sous la forme -1, 0 ou 1.

```rust
fn sign (& self) -> i32
```

## Paramètres, résultat et remarques

Renvoie i32/number. Le signe des zéros initialisés et non initialisés est 0 ; aucun signe distinct de zéro négatif n’est conservé.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-sign" locale="fr" />

## Erreurs et limites

Ces méthodes empruntent les entrées sans les consommer ni les modifier. L’opération de la bibliothèque ne renvoie pas Result. Le bridge rejette les handles libérés, déplacés ou provenant d’une autre exécution et applique la [politique de ressources](../resource-policy). Les exemples placent une sortie scalaire dans un champ `value` ; une sortie Decimal utilise son instantané complet.

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
