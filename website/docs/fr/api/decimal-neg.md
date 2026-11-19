---
locale: fr
source_revision: 84828af3d16c67c7fecd4ce94d987a7814ff2080c1c08e3527725c6cb1588c42
translation_provenance: en-US-french-reviewed
api_id: Decimal::neg
---
# Decimal::neg

Créer un décimal indépendant de signe opposé.

```rust
fn neg (& self) -> Self
```

## Paramètres, résultat et remarques

Renvoie Decimal en conservant l’échelle et les zéros finaux. Zéro reste numériquement zéro ; le résultat est initialisé même pour une entrée par défaut.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-neg" locale="fr" />

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
