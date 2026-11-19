---
locale: fr
source_revision: 784a138fd9dfff6c4bbabb3b837f6e63c10c904c5578943fe043993afcf99a55
translation_provenance: en-US-french-reviewed
api_id: Decimal::into_coefficient
---
# Decimal::into_coefficient

Extraire par déplacement le coefficient entier non mis à l’échelle.

```rust
fn into_coefficient (self) -> BigInt
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie BigInt en Rust et bigint en JS. L’échelle et l’état d’initialisation sont supprimés : 1.2300 donne donc 12300. Le zéro par défaut donne 0. Le Decimal est consommé et ne peut plus être lu ; utilisez coefficient pour un instantané sans consommation dans le bridge.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-into-coefficient" locale="fr" />

## Méthodes associées

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
