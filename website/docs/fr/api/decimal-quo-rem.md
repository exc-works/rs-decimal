---
locale: fr
source_revision: ab458b72e36df078d37dd6de4be21a55e10d1e6c955c47b4abcbe87cbbdd8e23
translation_provenance: en-US-french-reviewed
api_id: Decimal::quo_rem
---
# Decimal::quo_rem

Renvoyer un quotient entier et un reste Decimal signé.

```rust
fn quo_rem (& self , other : & Self) -> Result < (Self , Self) , DecimalError >
```

## Paramètres, résultat et utilisation courante

other : &Decimal ; Rust renvoie Result<(Decimal, Decimal), DecimalError>, représenté par [quotient, remainder]. Le quotient est tronqué vers zéro à l’échelle zéro. Le reste a le signe du dividende et la plus grande échelle des opérandes ; self = quotient * other + remainder. Chaque résultat est un objet enveloppe indépendant et initialisé ; les entrées restent utilisables. Un diviseur nul renvoie DivideByZero avant l’alignement.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-quo-rem" locale="fr" />

## Limites du navigateur

Respectez les règles de propriété ci-dessus. Un accès après déplacement renvoie une erreur moved ; free est idempotent. La vérification des types et des alias précède le déplacement, mais une erreur Rust ne restaure pas le récepteur consommé. La [politique de ressources](../resource-policy) est distincte des erreurs de la bibliothèque : les puissances contrôlent les valeurs intermédiaires et l’alignement contrôle la croissance des coefficients. Les contrôles conservateurs peuvent rejeter des calculs dont le résultat final est petit.

## Méthodes associées

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
