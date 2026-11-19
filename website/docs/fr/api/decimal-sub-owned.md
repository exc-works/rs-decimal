---
locale: fr
source_revision: ddfedf2b2a7b0914238752b884265b0c43b55c647cf39e728c8143e706a5b5ba
translation_provenance: en-US-french-reviewed
api_id: Decimal::sub_owned
---
# Decimal::sub_owned

Consommer l’opérande gauche pour un calcul exact.

```rust
fn sub_owned (self , other : & Self) -> Self
```

## Paramètres, résultat et utilisation courante

other : &Decimal ; renvoie un Decimal initialisé à la plus grande échelle des entrées. Le récepteur est déplacé, tandis que other reste utilisable. Le stockage peut être réutilisé, mais sa croissance peut allouer de la mémoire. Le bridge rejette l’utilisation du même objet enveloppe comme récepteur et comme other avant le déplacement. Après l’entrée dans Rust, même une erreur de ressources consomme le récepteur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-sub-owned" locale="fr" />

## Méthodes associées

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
