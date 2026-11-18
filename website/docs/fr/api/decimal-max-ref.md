---
locale: fr
source_revision: 78bc5479760ac53643504ad7a87ce54e9faf3966cce17b807b0ad11092850a19
translation_provenance: en-US-french-reviewed
api_id: Decimal::max_ref
---
# Decimal::max_ref

Sélectionner un opérande d’origine par comparaison numérique.

```rust
fn max_ref < 'a > (& 'a self , other : & 'a Self) -> & 'a Self
```

## Paramètres, résultat et utilisation courante

other : &Decimal ; Rust renvoie une référence partagée liée à la durée de vie de l’entrée. En cas d’égalité, self est choisi, avec son échelle et son initialisation. Le bridge renvoie l’objet enveloppe d’origine correspondant, pas une copie. Déplacer ou libérer cet objet invalide tous ses alias JavaScript ; JavaScript ne peut pas imposer les durées de vie des emprunts Rust. Utilisez max/min pour un résultat possédé indépendant.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-max-ref" locale="fr" />

## Limites du navigateur

Respectez les règles de propriété ci-dessus. Un accès après déplacement renvoie une erreur moved ; free est idempotent. La vérification des types et des alias précède le déplacement, mais une erreur Rust ne restaure pas le récepteur consommé. La [politique de ressources](../resource-policy) est distincte des erreurs de la bibliothèque : les puissances contrôlent les valeurs intermédiaires et l’alignement contrôle la croissance des coefficients. Les contrôles conservateurs peuvent rejeter des calculs dont le résultat final est petit.

## Méthodes associées

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
