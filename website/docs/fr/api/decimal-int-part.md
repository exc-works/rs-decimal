---
locale: fr
source_revision: 0b7e4ede82a7865b24f3ea40debe791505bf9ac255c8e2f8790aa1271d0fdf3a
translation_provenance: en-US-french-reviewed
api_id: Decimal::int_part
---
# Decimal::int_part

Extraire la partie entière en tronquant vers zéro.

```rust
fn int_part (& self) -> BigInt
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; Rust renvoie BigInt et l’API JavaScript bigint, sans conversion flottante. -12.75 donne -12, et non floor(-12.75) = -13. Le zéro par défaut donne zéro. L’exemple sérialise l’entier sous forme de chaîne pour préserver sa précision arbitraire dans JSON.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-int-part" locale="fr" />

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
