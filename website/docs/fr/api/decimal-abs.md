---
locale: fr
source_revision: c71c6b4ce225ea223944d584a5ec03b1adfe1dbd2adeb04f6beb38800f1e252e
translation_provenance: en-US-french-reviewed
api_id: Decimal::abs
---
# Decimal::abs

Créer un décimal indépendant contenant la valeur absolue.

```rust
fn abs (& self) -> Self
```

## Paramètres, résultat et remarques

Renvoie Decimal en conservant l’échelle et les zéros finaux. Le résultat est initialisé même si l’entrée est la valeur par défaut ; l’entrée reste inchangée.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-abs" locale="fr" />

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
