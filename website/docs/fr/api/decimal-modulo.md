---
locale: fr
source_revision: 561bf3d1118c3ce8a6c2b94160186e43d5ce2153ae6df6a545c57b9a2538ed41
translation_provenance: en-US-french-reviewed
api_id: Decimal::modulo
---
# Decimal::modulo

Renvoyer le reste signé de la division tronquée.

```rust
fn modulo (& self , other : & Self) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

other : &Decimal ; Rust renvoie Result<Decimal, DecimalError>. Le reste a le signe du dividende et la plus grande échelle des opérandes ; ce n’est pas un modulo euclidien non négatif. Un diviseur nul renvoie DivideByZero. Les entrées sont empruntées. Utilisez quo_rem pour obtenir le quotient et le reste.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-modulo" locale="fr" />

## Méthodes associées

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
