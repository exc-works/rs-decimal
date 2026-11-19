---
locale: fr
source_revision: 01c476851989c18cecc0970b361010eee7cb4cfd902d7e4240be18102abc0899
translation_provenance: en-US-french-reviewed
api_id: Decimal::power
---
# Decimal::power

Élever à une puissance entière en conservant l’échelle d’origine.

```rust
fn power (& self , power : i64) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

power : i64, transporté par bigint. Arrondit au pair à l’échelle de l’entrée ; les puissances négatives prennent l’inverse. 2.00 à la puissance -3 donne 0.12, tandis que 2 d’échelle entière à la puissance -3 donne 0. Zéro à la puissance zéro est la valeur 1 initialisée ; zéro à une puissance négative renvoie DivideByZero. Les exposants immenses dépassant la limite intermédiaire du cœur renvoient Overflow avant l’allocation. Le navigateur impose des budgets intermédiaires d’échelle et de coefficient plus petits ; 0 et +/-1 conservent les chemins rapides du cœur. L’entrée est empruntée.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-power" locale="fr" />

## Méthodes associées

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
