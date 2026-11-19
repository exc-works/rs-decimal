---
locale: fr
source_revision: 1534eba12b3e65df06a4bea76e8dd9c9cd12eeb6bcfbce92f69f83b923eef4c3
translation_provenance: en-US-french-reviewed
api_id: "ScanValue::Int64"
---
# ScanValue::Int64

Choisir la variante d’entrée SQL Int64.

```rust
Int64 (i64)
```

## Paramètres, résultat et utilisation courante

Transporte un i64 exactement. Le navigateur exige un bigint dans la plage signée sur 64 bits ; un number JavaScript est rejeté. Les variantes sont empruntées par scan et peuvent être réutilisées dans la même exécution du navigateur ; les valeurs fabriquées ou provenant d’une autre exécution sont rejetées.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-int64" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
