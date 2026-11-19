---
locale: fr
source_revision: 3a7d890770468c269032ca95ee3365a1f5fdeb70051590bf674c733d1e1658d6
translation_provenance: en-US-french-reviewed
api_id: Decimal::is_zero
---
# Decimal::is_zero

Tester le zéro numérique avant une opération dépendant de la valeur.

```rust
fn is_zero (& self) -> bool
```

## Paramètres, résultat et remarques

Renvoie bool/boolean, et également true pour la valeur par défaut non initialisée. Utilisez is_nil() si l’état d’initialisation importe.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-zero" locale="fr" />

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
