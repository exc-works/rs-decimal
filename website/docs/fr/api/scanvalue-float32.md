---
locale: fr
source_revision: 15c94f02827b09a573140cb09fd7743d1624bd74d93875d8d0450c89214f2677
translation_provenance: en-US-french-reviewed
api_id: "ScanValue::Float32"
---
# ScanValue::Float32

Choisir la variante d’entrée SQL Float32.

```rust
Float32 (f32)
```

## Paramètres, résultat et utilisation courante

Transporte binary32. Le navigateur accepte number et Rust le réduit en f32 avant scan. NaN, les infinis et un dépassement lors de la réduction produisent Unmarshal. Les variantes sont empruntées par scan et peuvent être réutilisées dans la même exécution du navigateur ; les valeurs fabriquées ou provenant d’une autre exécution sont rejetées.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-float32" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
