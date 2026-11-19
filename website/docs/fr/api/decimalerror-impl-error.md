---
locale: fr
source_revision: d3605c64b7e680ec58960c25358a321ed68015b81953ff878939d179147793d3
translation_provenance: en-US-french-reviewed
api_id: "DecimalError::impl Error"
---
# DecimalError::impl Error

Utiliser l’implémentation Rust Error.

```rust
impl Error for DecimalError
```

## Paramètres, résultat et utilisation courante

À utiliser comme Error Rust standard, y compris pour la propagation d’erreurs encapsulées dans Box. Les catégories supplémentaires ne sont pas une chaîne de causes ; source() vaut None. C’est une syntaxe de trait Rust natif ; le navigateur ne l’expose pas comme méthode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-error" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
