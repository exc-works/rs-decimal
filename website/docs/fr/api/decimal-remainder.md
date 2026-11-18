---
locale: fr
source_revision: d4492d82b410cc5d7b67274496227b1fa1d6279b6d276d6bd8bfbe25ef422dd4
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

## Limites du navigateur

Respectez les règles de propriété ci-dessus. Un accès après déplacement renvoie une erreur moved ; free est idempotent. La vérification des types et des alias précède le déplacement, mais une erreur Rust ne restaure pas le récepteur consommé. La [politique de ressources](../resource-policy) est distincte des erreurs de la bibliothèque : les puissances contrôlent les valeurs intermédiaires et l’alignement contrôle la croissance des coefficients. Les contrôles conservateurs peuvent rejeter des calculs dont le résultat final est petit.

## Méthodes associées

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
