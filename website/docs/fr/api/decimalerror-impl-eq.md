---
locale: fr
source_revision: cbf84996c156999f0cfb9cac49a1135ecf0d19ca8aba3abbedbed0acff443c0a
translation_provenance: en-US-french-reviewed
api_id: "DecimalError::impl Eq"
---
# DecimalError::impl Eq

Utiliser l’implémentation Rust Eq.

```rust
impl Eq for DecimalError
```

## Paramètres, résultat et utilisation courante

L’égalité compare le message et les catégories complets de l’erreur, ou la variante exacte de l’énumération. L’égalité de DecimalError est plus stricte que l’appartenance à une catégorie. C’est une syntaxe de trait Rust natif ; le navigateur ne l’expose pas comme méthode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-eq" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
