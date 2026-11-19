---
locale: fr
source_revision: ffab3b21dcd7c80e0222f2cd0457116e79f56aafc95fc2ed3c796e423b9661cc
translation_provenance: en-US-french-reviewed
api_id: Decimal::new_from_big_rat
---
# Decimal::new_from_big_rat

Convertir un rationnel en un décimal exact à développement fini.

```rust
fn new_from_big_rat (value : & BigRational) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

value : &BigRational ; l’API JavaScript accepte {numerator: bigint, denominator: bigint}. Le dénominateur doit être non nul, sinon une erreur de paramètre de l’API JavaScript est produite ; Rust BigRational normalise les signes et réduit les facteurs communs. Seuls les dénominateurs réduits contenant uniquement les facteurs 2 et 5 produisent un développement fini ; d’autres facteurs renvoient InvalidFormat. Le résultat est initialisé. Les contrôles du navigateur interviennent après réduction et limitent l’échelle de sortie et la croissance du coefficient avant la conversion du cœur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-big-rat" locale="fr" />

## Méthodes associées

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
