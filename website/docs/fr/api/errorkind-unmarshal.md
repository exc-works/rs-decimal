---
locale: fr
source_revision: 274359ec57012f0ef20672a2e83c98afd2d82df0c75391b519022c4925e09ca8
translation_provenance: en-US-french-reviewed
api_id: "ErrorKind::Unmarshal"
---
# ErrorKind::Unmarshal

Identifier les erreurs Unmarshal.

```rust
Unmarshal
```

## Paramètres, résultat et utilisation courante

Le décodage a échoué. Une autre catégorie, comme InvalidFormat, peut en préciser la cause ; testez l’appartenance plutôt que de supposer une catégorie unique. ErrorKind est une catégorie stable, pas un message textuel. Les valeurs de l’énumération du navigateur sont les noms des variantes Rust ; les erreurs construites utilisent matches(), tandis que les exceptions interceptées du bridge exposent un tableau kinds.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-unmarshal" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
