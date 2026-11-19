---
locale: fr
source_revision: 8e571c100355b6f464b7992d5d9d515f9f9923a4538f1e43b8cc48af4b41a4c7
translation_provenance: en-US-french-reviewed
api_id: Decimal::coefficient
---
# Decimal::coefficient

Inspecter le coefficient entier exact non mis à l’échelle.

```rust
fn coefficient (& self) -> & BigInt
```

## Paramètres, résultat et remarques

Rust emprunte &BigInt ; l’API JavaScript renvoie un instantané bigint immuable, pas une référence reflétant les modifications. Le coefficient inclut les zéros finaux et n’est pas la partie entière du décimal. L’exemple affiche bigint sous forme de texte pour conserver tous les chiffres dans JSON.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-coefficient" locale="fr" />

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
