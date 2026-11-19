---
locale: fr
source_revision: 5ee0e7b51aa068361991bd8a8b6016ae9f25f271ec57ec1d0990b39445f68b64
translation_provenance: en-US-french-reviewed
api_id: "ErrorKind::impl PartialEq"
---
# ErrorKind::impl PartialEq

Utiliser l’implémentation Rust PartialEq.

```rust
impl PartialEq for ErrorKind
```

## Paramètres, résultat et utilisation courante

L’égalité compare le message et les catégories complets de l’erreur, ou la variante exacte de l’énumération. L’égalité de DecimalError est plus stricte que l’appartenance à une catégorie. C’est une syntaxe de trait Rust natif ; le navigateur ne l’expose pas comme méthode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-partialeq" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
