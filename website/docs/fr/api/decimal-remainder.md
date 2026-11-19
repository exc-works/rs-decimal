---
locale: fr
source_revision: 64d94a480e185cbb38c975ef60712d5ae4ac373961962b675392db6ef9375f09
translation_provenance: en-US-french-reviewed
api_id: Decimal::remainder
---
# Decimal::remainder

Séparer la partie entière et le reste fractionnaire non mis à l’échelle.

```rust
fn remainder (& self) -> (BigInt , BigInt)
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie (BigInt, BigInt), représenté par [bigint, bigint]. La première valeur est tronquée vers zéro ; la seconde est le reste signé du coefficient, pas une fraction Decimal. Pour -12.750, le résultat est (-12, -750) : la seconde valeur nécessite l’échelle d’origine 3 pour représenter -0.750. Le zéro par défaut renvoie (0, 0).

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-remainder" locale="fr" />

## Méthodes associées

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
