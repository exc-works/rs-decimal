---
locale: fr
source_revision: cd78c2dcf4b67b04ef802fd73ba3ef0da02aad24d1bb17be5cc5df5d221df3cd
translation_provenance: en-US-french-reviewed
api_id: Decimal::bit_len
---
# Decimal::bit_len

Compter les bits de la valeur absolue du coefficient non mis à l’échelle.

```rust
fn bit_len (& self) -> usize
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; Rust renvoie usize, l’API JavaScript renvoie number. L’échelle n’est pas appliquée et le bit de signe n’est pas compté : -1.20 a pour coefficient -120 et pour longueur en bits 7. Zéro, y compris le zéro par défaut, renvoie 0. Ce n’est pas l’indice du bit de poids fort à 1.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-bit-len" locale="fr" />

## Méthodes associées

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
