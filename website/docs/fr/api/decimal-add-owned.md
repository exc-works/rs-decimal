---
locale: fr
source_revision: 851de95326d3b1f10839fcbae723fc6950eca60bcbb6e43b9e406040ff806dc1
translation_provenance: en-US-french-reviewed
api_id: Decimal::add_owned
---
# Decimal::add_owned

Consommer l’opérande gauche pour un calcul exact.

```rust
fn add_owned (self , other : & Self) -> Self
```

## Paramètres, résultat et utilisation courante

other : &Decimal ; renvoie un Decimal initialisé à la plus grande échelle des entrées. Le récepteur est déplacé, tandis que other reste utilisable. Le stockage peut être réutilisé, mais sa croissance peut allouer de la mémoire. Le bridge rejette l’utilisation du même objet enveloppe comme récepteur et comme other avant le déplacement. Après l’entrée dans Rust, même une erreur de ressources consomme le récepteur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-add-owned" locale="fr" />

## Méthodes associées

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
