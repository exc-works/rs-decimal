---
locale: fr
source_revision: b276bcddff5985f20e7905654e4b3cef6f352b3cfa139550a92b9425bd777e62
translation_provenance: en-US-french-reviewed
api_id: "ScanValue::Float64"
---
# ScanValue::Float64

Choisir la variante d’entrée SQL Float64.

```rust
Float64 (f64)
```

## Paramètres, résultat et utilisation courante

Transporte binary64. Les valeurs finies utilisent la conversion décimale la plus courte du cœur ; NaN et les infinis renvoient Unmarshal. Des number JavaScript déjà arrondis ne permettent pas de retrouver les chiffres d’origine. Les variantes sont empruntées par scan et peuvent être réutilisées dans la même exécution du navigateur ; les valeurs fabriquées ou provenant d’une autre exécution sont rejetées.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-float64" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
