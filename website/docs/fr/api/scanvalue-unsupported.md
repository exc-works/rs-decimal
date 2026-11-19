---
locale: fr
source_revision: 78277d1082dac8a98f79b7018413113c09afb0c224a6c82ae6c179432b0517f1
translation_provenance: en-US-french-reviewed
api_id: "ScanValue::Unsupported"
---
# ScanValue::Unsupported

Choisir la variante d’entrée SQL Unsupported.

```rust
Unsupported
```

## Paramètres, résultat et utilisation courante

Signale une entrée de pilote non prise en charge sans conversion forcée. scan renvoie Unmarshal et préserve la valeur existante. Les variantes sont empruntées par scan et peuvent être réutilisées dans la même exécution du navigateur ; les valeurs fabriquées ou provenant d’une autre exécution sont rejetées.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-unsupported" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
