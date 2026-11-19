---
locale: fr
source_revision: 7388877c082aab40b6cea1eaae69162f25e5c07d8d3673378f0fdd23fef3e494
translation_provenance: en-US-french-reviewed
api_id: Decimal::scale
---
# Decimal::scale

Lire l’échelle décimale non négative stockée.

```rust
fn scale (& self) -> usize
```

## Paramètres, résultat et remarques

Renvoie usize sous forme d’un number exact dans la limite d’échelle du playground. La valeur est identique à precision().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-scale" locale="fr" />

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
