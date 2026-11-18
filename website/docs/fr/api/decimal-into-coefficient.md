---
locale: fr
source_revision: fa1a3656d76ca2759cc49aed5b71589136f0ed7e84a57f811be7062ea9aa8860
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

## Limites du navigateur

Respectez les règles de propriété ci-dessus. Un accès après déplacement renvoie une erreur moved ; free est idempotent. La vérification des types et des alias précède le déplacement, mais une erreur Rust ne restaure pas le récepteur consommé. La [politique de ressources](../resource-policy) est distincte des erreurs de la bibliothèque : les puissances contrôlent les valeurs intermédiaires et l’alignement contrôle la croissance des coefficients. Les contrôles conservateurs peuvent rejeter des calculs dont le résultat final est petit.

## Méthodes associées

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
