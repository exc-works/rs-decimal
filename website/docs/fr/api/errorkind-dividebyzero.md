---
locale: fr
source_revision: c5bc8b97bab18c557c0918fb7ad15cabc2fd824c618c469b8f007d1f6299272b
translation_provenance: en-US-french-reviewed
api_id: "ErrorKind::DivideByZero"
---
# ErrorKind::DivideByZero

Identifier les erreurs DivideByZero.

```rust
DivideByZero
```

## Paramètres, résultat et utilisation courante

Un diviseur est nul. ErrorKind est une catégorie stable, pas un message textuel. Les valeurs de l’énumération du navigateur sont les noms des variantes Rust ; les erreurs construites utilisent matches(), tandis que les exceptions interceptées du bridge exposent un tableau kinds.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-dividebyzero" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
