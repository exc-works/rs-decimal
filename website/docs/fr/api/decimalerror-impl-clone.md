---
locale: fr
source_revision: 9dbbcca20e9ec70948f275a66820b5aebd2bfa7f33ae4f382b6d22c025c682c3
translation_provenance: en-US-french-reviewed
api_id: "DecimalError::impl Clone"
---
# DecimalError::impl Clone

Utiliser l’implémentation Rust Clone.

```rust
impl Clone for DecimalError
```

## Paramètres, résultat et utilisation courante

Clone conserve la valeur complète ; les tranches ScanValue empruntées font toujours référence aux données d’origine. C’est une syntaxe de trait Rust natif ; le navigateur ne l’expose pas comme méthode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-clone" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
