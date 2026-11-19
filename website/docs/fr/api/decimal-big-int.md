---
locale: fr
source_revision: f7f692709bde775f34d7f5feb756c59944ff77c763bae1798b2b4306d9770c41
translation_provenance: en-US-french-reviewed
api_id: Decimal::big_int
---
# Decimal::big_int

Copier le coefficient entier exact non mis à l’échelle.

```rust
fn big_int (& self) -> BigInt
```

## Paramètres, résultat et remarques

Rust renvoie un BigInt indépendant ; l’API JavaScript renvoie bigint. Les zéros finaux sont inclus, contrairement à une conversion entière par troncature. L’exemple affiche bigint sous forme de texte pour JSON.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-big-int" locale="fr" />

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
