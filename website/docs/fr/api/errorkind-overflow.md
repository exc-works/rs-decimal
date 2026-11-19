---
locale: fr
source_revision: 368adfaeea00a469d274e0e87fc3e84f66ca2848d32cfd8759858b2bac06fe99
translation_provenance: en-US-french-reviewed
api_id: "ErrorKind::Overflow"
---
# ErrorKind::Overflow

Identifier les erreurs Overflow.

```rust
Overflow
```

## Paramètres, résultat et utilisation courante

Un coefficient, une échelle ou une opération dépasse la plage prise en charge par le cœur. ErrorKind est une catégorie stable, pas un message textuel. Les valeurs de l’énumération du navigateur sont les noms des variantes Rust ; les erreurs construites utilisent matches(), tandis que les exceptions interceptées du bridge exposent un tableau kinds.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-overflow" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
