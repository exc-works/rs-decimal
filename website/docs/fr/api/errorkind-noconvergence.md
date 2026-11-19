---
locale: fr
source_revision: aeb152b6bffb5f406cc2fbe512dd2449186121fa1cd03efbf5d74f0d3891aed1
translation_provenance: en-US-french-reviewed
api_id: "ErrorKind::NoConvergence"
---
# ErrorKind::NoConvergence

Identifier les erreurs NoConvergence.

```rust
NoConvergence
```

## Paramètres, résultat et utilisation courante

L’algorithme borné n’a pas pu certifier un résultat arrondi ; cela ne signifie pas que le résultat mathématique n’existe pas. ErrorKind est une catégorie stable, pas un message textuel. Les valeurs de l’énumération du navigateur sont les noms des variantes Rust ; les erreurs construites utilisent matches(), tandis que les exceptions interceptées du bridge exposent un tableau kinds.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-noconvergence" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
