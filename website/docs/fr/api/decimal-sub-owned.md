---
locale: fr
source_revision: 347ad719aa4b33bd925c7dfe63b5e557c57b390b4d8d0b57d7aff0d9ca62ac93
translation_provenance: en-US-french-reviewed
api_id: Decimal::sub_owned
---
# Decimal::sub_owned

Consommer l’opérande gauche pour un calcul exact.

```rust
fn sub_owned (self , other : & Self) -> Self
```

## Paramètres, résultat et utilisation courante

other : &Decimal ; renvoie un Decimal initialisé à la plus grande échelle des entrées. Le récepteur est déplacé, tandis que other reste utilisable. Le stockage peut être réutilisé, mais sa croissance peut allouer de la mémoire. L’API JavaScript rejette l’utilisation du même objet enveloppe comme récepteur et comme other avant le déplacement. Après l’entrée dans Rust, même une erreur de ressources consomme le récepteur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-sub-owned" locale="fr" />

## Méthodes associées

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
