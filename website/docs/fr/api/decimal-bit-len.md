---
locale: fr
source_revision: 3cc2ce52c4d0ed0d111f2e4df3eea8f2b90cb54656fa97590458dae94eb7b967
translation_provenance: en-US-french-reviewed
api_id: Decimal::bit_len
---
# Decimal::bit_len

Compter les bits de la valeur absolue du coefficient non mis à l’échelle.

```rust
fn bit_len (& self) -> usize
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; Rust renvoie usize, le bridge renvoie number. L’échelle n’est pas appliquée et le bit de signe n’est pas compté : -1.20 a pour coefficient -120 et pour longueur en bits 7. Zéro, y compris le zéro par défaut, renvoie 0. Ce n’est pas l’indice du bit de poids fort à 1.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-bit-len" locale="fr" />

## Méthodes associées

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
