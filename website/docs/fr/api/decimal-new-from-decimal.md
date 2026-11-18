---
locale: fr
source_revision: 37b6ebebec69abdf2809e71eb7e3cad7353a792a069c7217103a51f2eea16c7f
translation_provenance: en-US-french-reviewed
api_id: Decimal::new_from_decimal
---
# Decimal::new_from_decimal

Copier un Decimal existant, y compris sa représentation stockée et son état d’initialisation.

```rust
fn new_from_decimal (value : & Self) -> Self
```

## Paramètres et résultat

`value: &Decimal` est emprunté. Le Decimal renvoyé est indépendant ; l’entrée reste utilisable.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-decimal" locale="fr" />

## Remarques et limites

Copier `Decimal::default()` conserve l’état non initialisé ; cela ne le transforme pas en zéro initialisé. En JavaScript, fournissez un Decimal encore valide provenant de la même exécution. Ce constructeur ne le consomme pas.

Le navigateur applique également la [politique de ressources](../resource-policy). Les entrées JavaScript sont validées avant l’appel à Rust.

## Méthodes associées

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
