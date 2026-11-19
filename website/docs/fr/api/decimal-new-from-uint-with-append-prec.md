---
locale: fr
source_revision: b1a9d67b6db859834f3ef9cd2d0fefcd4239bbf10472aedba8e0a86a95b51eeb
translation_provenance: en-US-french-reviewed
api_id: Decimal::new_from_uint_with_append_prec
---
# Decimal::new_from_uint_with_append_prec

Conserver la valeur numérique d’un entier non signé en ajoutant des décimales stockées.

```rust
fn new_from_uint_with_append_prec (value : u64 , prec : isize) -> Result < Self , DecimalError >
```

## Paramètres et résultat

`value: u64` → `bigint` non négatif ; `prec: isize` → `number` entier. Le résultat conserve la valeur entière à l’échelle `prec`.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-uint-with-append-prec" locale="fr" />

## Conseils d’utilisation

Les valeurs négatives ou hors plage sont des erreurs de paramètre de l’API JavaScript. Une précision négative produit `InvalidPrecision`. Examinez la sortie avec zéros finaux pour voir les décimales ajoutées.

## Méthodes associées

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
