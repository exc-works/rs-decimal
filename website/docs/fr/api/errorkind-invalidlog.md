---
locale: fr
source_revision: c456812ef50493713a2bbd81d6c57db5a0f730099b5878e83b3d4f358245d822
translation_provenance: en-US-french-reviewed
api_id: "ErrorKind::InvalidLog"
---
# ErrorKind::InvalidLog

Identifier les erreurs InvalidLog.

```rust
InvalidLog
```

## Paramètres, résultat et utilisation courante

L’entrée d’un logarithme n’est pas strictement positive. ErrorKind est une catégorie stable, pas un message textuel. Les valeurs de l’énumération du navigateur sont les noms des variantes Rust ; les erreurs construites utilisent matches(), tandis que les exceptions interceptées du bridge exposent un tableau kinds.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidlog" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
