---
locale: fr
source_revision: c50c700bf613d916c3c5e5c98c1711b754a7a2e86cbb6f64e10b64167cb79a2a
translation_provenance: en-US-french-reviewed
api_id: "ErrorKind::NoConvergence"
---
# ErrorKind::NoConvergence

Identifier les erreurs NoConvergence.

```rust
NoConvergence
```

## Paramètres, résultat et utilisation courante

L’algorithme borné n’a pas pu certifier un résultat arrondi ; cela ne signifie pas que le résultat mathématique n’existe pas.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-noconvergence" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
