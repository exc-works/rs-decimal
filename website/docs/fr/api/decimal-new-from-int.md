---
locale: fr
source_revision: aad24e4a42d458d995751545010904db4f3e9453abb0fa3c66514c8c59b55494
translation_provenance: en-US-french-reviewed
api_id: Decimal::new_from_int
---
# Decimal::new_from_int

Créer un entier initialisé à partir d’un entier signé de largeur native.

```rust
fn new_from_int (value : isize) -> Self
```

## Paramètres et résultat

`value: isize` → un `number` entier JavaScript ; l’échelle du résultat est zéro.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-int" locale="fr" />

## Remarques et limites

La cible du navigateur est wasm32 : la plage acceptée va donc de -2147483648 à 2147483647, même si Rust natif s’exécute sur une machine 64 bits. Les fractions, NaN et l’infini sont des erreurs de paramètre.

Le navigateur applique également la [politique de ressources](../resource-policy). Les entrées JavaScript sont validées avant l’appel à Rust.

## Méthodes associées

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
