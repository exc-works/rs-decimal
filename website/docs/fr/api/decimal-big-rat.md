---
locale: fr
source_revision: 3e67428ba1ea89053920f2f5796e826dd297f81c12e673f6f4e20f40216c28ff
translation_provenance: en-US-french-reviewed
api_id: Decimal::big_rat
---
# Decimal::big_rat

Lire la valeur rationnelle exacte réduite.

```rust
fn big_rat (& self) -> BigRational
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; Rust renvoie BigRational. Le bridge renvoie un instantané immuable {numerator: bigint, denominator: bigint}, normalisé avec un dénominateur positif. L’échelle décimale et les zéros finaux ne sont pas conservés comme métadonnées rationnelles ; zéro devient 0/1. L’entrée est empruntée et inchangée.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-big-rat" locale="fr" />

## Méthodes associées

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
