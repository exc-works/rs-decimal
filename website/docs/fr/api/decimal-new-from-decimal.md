---
locale: fr
source_revision: 3e32f687004e72286fa606992f7977ccbc210f3d36756c6006ab536cd5091f5a
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

## Conseils d’utilisation

Copier `Decimal::default()` conserve l’état non initialisé ; cela ne le transforme pas en zéro initialisé. En JavaScript, fournissez un Decimal encore valide provenant de la même exécution. Ce constructeur ne le consomme pas.

## Méthodes associées

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
