---
locale: fr
source_revision: cd0b0af7153d96e3d2f3f7126c5b673c301639179f9c4225b3dd81e2a74678a4
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

## Limites du navigateur

Respectez les règles de propriété ci-dessus. Un accès après déplacement renvoie une erreur moved ; free est idempotent. La vérification des types et des alias précède le déplacement, mais une erreur Rust ne restaure pas le récepteur consommé. La [politique de ressources](../resource-policy) est distincte des erreurs de la bibliothèque : les puissances contrôlent les valeurs intermédiaires et l’alignement contrôle la croissance des coefficients. Les contrôles conservateurs peuvent rejeter des calculs dont le résultat final est petit.

## Méthodes associées

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
