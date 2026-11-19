---
locale: fr
source_revision: e14d22e2fbe70e02eda750145ed5b1a70252e171e63aeaafcfc01281be24e818
translation_provenance: en-US-french-reviewed
api_id: "ErrorKind::InvalidArgument"
---
# ErrorKind::InvalidArgument

Identifier les erreurs InvalidArgument.

```rust
InvalidArgument
```

## Paramètres, résultat et utilisation courante

Un argument, une règle de validation ou un modèle de message est invalide. ErrorKind est une catégorie stable, pas un message textuel. Les valeurs de l’énumération du navigateur sont les noms des variantes Rust ; les erreurs construites utilisent matches(), tandis que les exceptions interceptées du bridge exposent un tableau kinds.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidargument" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
