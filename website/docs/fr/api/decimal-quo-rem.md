---
locale: fr
source_revision: 10c5c67f0eca13950935316848dbaf7b4f6fbe37a761aaabb0a34ec3fe68d1c2
translation_provenance: en-US-french-reviewed
api_id: Decimal::quo_rem
---
# Decimal::quo_rem

Renvoyer un quotient entier et un reste Decimal signé.

```rust
fn quo_rem (& self , other : & Self) -> Result < (Self , Self) , DecimalError >
```

## Paramètres, résultat et utilisation courante

other : &Decimal ; Rust renvoie Result<(Decimal, Decimal), DecimalError>, représenté par [quotient, remainder]. Le quotient est tronqué vers zéro à l’échelle zéro. Le reste a le signe du dividende et la plus grande échelle des opérandes ; self = quotient * other + remainder. Chaque résultat est un objet enveloppe indépendant et initialisé ; les entrées restent utilisables. Un diviseur nul renvoie DivideByZero avant l’alignement.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-quo-rem" locale="fr" />

## Méthodes associées

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
