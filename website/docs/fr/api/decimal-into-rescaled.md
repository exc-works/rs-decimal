---
locale: fr
source_revision: aed80a094ad154f3abfbbdc72125d175f579082056ef3b224d8dda331e64cff5
translation_provenance: en-US-french-reviewed
api_id: Decimal::into_rescaled
---
# Decimal::into_rescaled

Consommer un décimal en modifiant son nombre de décimales.

```rust
fn into_rescaled (self , prec : isize , mode : RoundingMode) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

prec : isize ; mode : RoundingMode. Comme rescale, cette méthode arrondit ou ajoute des zéros et initialise toujours le résultat. Le récepteur est déplacé même lorsque Rust renvoie InvalidPrecision, RoundUnnecessary ou une erreur de ressources du navigateur. Un type ou une plage JavaScript incorrects sont rejetés avant l’entrée dans Rust et ne le consomment pas.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-into-rescaled" locale="fr" />

## Méthodes associées

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
