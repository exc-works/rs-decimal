---
locale: fr
source_revision: bceeb7ede6fc6a0450eec9bb9848cbe43ca4c0d990bf02edf9131f531001c34f
translation_provenance: en-US-french-reviewed
api_id: "DecimalError::impl PartialEq"
---
# DecimalError::impl PartialEq

Utiliser l’implémentation Rust PartialEq.

```rust
impl PartialEq for DecimalError
```

## Paramètres, résultat et utilisation courante

L’égalité compare le message et les catégories complets de l’erreur, ou la variante exacte de l’énumération. L’égalité de DecimalError est plus stricte que l’appartenance à une catégorie. C’est une syntaxe de trait Rust natif ; le navigateur ne l’expose pas comme méthode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-partialeq" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
