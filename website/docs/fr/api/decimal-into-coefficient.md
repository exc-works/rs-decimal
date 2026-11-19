---
locale: fr
source_revision: cc52467a55bcaa7e86e52d9cf523d5b8a22924e126619052d59e51cd0b6aab8b
translation_provenance: en-US-french-reviewed
api_id: Decimal::into_coefficient
---
# Decimal::into_coefficient

Extraire par déplacement le coefficient entier non mis à l’échelle.

```rust
fn into_coefficient (self) -> BigInt
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie BigInt en Rust et bigint en JS. L’échelle et l’état d’initialisation sont supprimés : 1.2300 donne donc 12300. Le zéro par défaut donne 0. Le Decimal est consommé et ne peut plus être lu ; utilisez coefficient pour un instantané sans consommation en JavaScript.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-into-coefficient" locale="fr" />

## Méthodes associées

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
