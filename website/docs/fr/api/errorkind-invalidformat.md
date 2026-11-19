---
locale: fr
source_revision: 72523519c8ccf1aad4c8baebb6ac1ef42e8070b9cc7f7f72b600ff77781eaf4e
translation_provenance: en-US-french-reviewed
api_id: "ErrorKind::InvalidFormat"
---
# ErrorKind::InvalidFormat

Identifier les erreurs InvalidFormat.

```rust
InvalidFormat
```

## Paramètres, résultat et utilisation courante

L’entrée n’est pas une représentation décimale finie prise en charge. ErrorKind est une catégorie stable, pas un message textuel. Les valeurs de l’énumération du navigateur sont les noms des variantes Rust ; les erreurs construites utilisent matches(), tandis que les exceptions interceptées du bridge exposent un tableau kinds.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidformat" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
