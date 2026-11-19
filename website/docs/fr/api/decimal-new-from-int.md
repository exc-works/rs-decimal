---
locale: fr
source_revision: dd3a6777ec0d2daf335730b8ca67dd44bd500850bf5f5a894aa8a18b1d5c40f9
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

## Conseils d’utilisation

La cible du navigateur est wasm32 : la plage acceptée va donc de -2147483648 à 2147483647, même si Rust natif s’exécute sur une machine 64 bits. Les fractions, NaN et l’infini sont des erreurs de paramètre.

## Méthodes associées

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
