---
locale: fr
source_revision: 7d0c030a450a67fa4b0e618dbc6cb501daeb5fbc64647ae39a02ca84be312b44
translation_provenance: en-US-french-reviewed
api_id: Decimal::is_nil
---
# Decimal::is_nil

Déterminer si un décimal est la valeur par défaut non initialisée.

```rust
fn is_nil (& self) -> bool
```

## Paramètres, résultat et remarques

Renvoie bool/boolean. La valeur par défaut n’est pas initialisée, tandis qu’un zéro construit l’est ; les deux sont numériquement égaux. Ce n’est pas l’indicateur de validité de NullDecimal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-nil" locale="fr" />

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
