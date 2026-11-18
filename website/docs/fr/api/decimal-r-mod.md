---
locale: fr
source_revision: 93dfb354f7906fd9c23e3b9ad99dace81c54c1bb0757216e3e8aa83d274f8bb9
translation_provenance: en-US-french-reviewed
api_id: Decimal::r#mod
---
# Decimal::r#mod

Renvoyer le reste signé de la division tronquée.

```rust
fn r#mod (& self , other : & Self) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

other : &Decimal ; Rust renvoie Result<Decimal, DecimalError>. Le reste a le signe du dividende et la plus grande échelle des opérandes ; ce n’est pas un modulo euclidien non négatif. Un diviseur nul renvoie DivideByZero. Les entrées sont empruntées. Rust exige r#mod car mod est un mot-clé ; JavaScript utilise .mod(other). Les deux noms appellent le même comportement du cœur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-r-mod" locale="fr" />

## Limites du navigateur

Respectez les règles de propriété ci-dessus. Un accès après déplacement renvoie une erreur moved ; free est idempotent. La vérification des types et des alias précède le déplacement, mais une erreur Rust ne restaure pas le récepteur consommé. La [politique de ressources](../resource-policy) est distincte des erreurs de la bibliothèque : les puissances contrôlent les valeurs intermédiaires et l’alignement contrôle la croissance des coefficients. Les contrôles conservateurs peuvent rejeter des calculs dont le résultat final est petit.

## Méthodes associées

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
