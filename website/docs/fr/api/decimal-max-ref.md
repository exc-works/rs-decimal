---
locale: fr
source_revision: 52eca22e980279ceae03e771dce435ca3ba3fd74042d6f5541b41c478a23c74f
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

## Méthodes associées

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
