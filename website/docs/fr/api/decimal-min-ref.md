---
locale: fr
source_revision: c991ff009b87a451c4d3a01a7e163c4b8e995107fc18ca0a0ef4ec120e3aee71
translation_provenance: en-US-french-reviewed
api_id: Decimal::min_ref
---
# Decimal::min_ref

Sélectionner un opérande d’origine par comparaison numérique.

```rust
fn min_ref < 'a > (& 'a self , other : & 'a Self) -> & 'a Self
```

## Paramètres, résultat et utilisation courante

other : &Decimal ; Rust renvoie une référence partagée liée à la durée de vie de l’entrée. En cas d’égalité, self est choisi, avec son échelle et son initialisation. L’API JavaScript renvoie l’objet enveloppe d’origine correspondant, pas une copie. Déplacer ou libérer cet objet invalide tous ses alias JavaScript ; JavaScript ne peut pas imposer les durées de vie des emprunts Rust. Utilisez max/min pour un résultat possédé indépendant.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-min-ref" locale="fr" />

## Méthodes associées

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
