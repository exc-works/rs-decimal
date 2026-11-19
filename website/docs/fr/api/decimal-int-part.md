---
locale: fr
source_revision: b7ff1544694c51babb7af7cb99735ecc6687d86e9cd9a108af321bed93252beb
translation_provenance: en-US-french-reviewed
api_id: Decimal::int_part
---
# Decimal::int_part

Extraire la partie entière en tronquant vers zéro.

```rust
fn int_part (& self) -> BigInt
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; Rust renvoie BigInt et le bridge bigint, sans conversion flottante. -12.75 donne -12, et non floor(-12.75) = -13. Le zéro par défaut donne zéro. L’exemple sérialise l’entier sous forme de chaîne pour préserver sa précision arbitraire dans JSON.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-int-part" locale="fr" />

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
