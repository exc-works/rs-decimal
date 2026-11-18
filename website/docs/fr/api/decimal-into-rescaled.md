---
locale: fr
source_revision: f56b3cc9eb8667e7cf067d7d1256d88fafb84d3cb5b89748b59c7df428bab4cf
translation_provenance: en-US-french-reviewed
api_id: Decimal::into_rescaled
---
# Decimal::into_rescaled

Consommer un décimal en modifiant son nombre de décimales.

```rust
fn into_rescaled (self , prec : isize , mode : RoundingMode) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

prec : isize ; mode : RoundingMode. Comme rescale, cette méthode arrondit ou ajoute des zéros et initialise toujours le résultat. Le récepteur est déplacé même lorsque Rust renvoie InvalidPrecision, RoundUnnecessary ou une erreur de ressources du navigateur. Un type ou une plage JavaScript incorrects sont rejetés avant l’entrée dans Rust et ne le consomment pas.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-into-rescaled" locale="fr" />

## Limites du navigateur

Respectez les règles de propriété ci-dessus. Un accès après déplacement renvoie une erreur moved ; free est idempotent. La vérification des types et des alias précède le déplacement, mais une erreur Rust ne restaure pas le récepteur consommé. La [politique de ressources](../resource-policy) est distincte des erreurs de la bibliothèque : les puissances contrôlent les valeurs intermédiaires et l’alignement contrôle la croissance des coefficients. Les contrôles conservateurs peuvent rejeter des calculs dont le résultat final est petit.

## Méthodes associées

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
