---
locale: fr
source_revision: 2f8f701187893878a876b218458ba92b8464c997083c08e62c584ef14b118454
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

## Remarques et limites

Les valeurs négatives ou hors plage sont des erreurs de paramètre du bridge. Une précision négative produit `InvalidPrecision` ; une précision trop grande atteint la limite de ressources distincte du playground. Examinez la sortie avec zéros finaux pour voir les décimales ajoutées.

Le navigateur applique également la [politique de ressources](../resource-policy). Les entrées JavaScript sont validées avant l’appel à Rust.

## Méthodes associées

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
