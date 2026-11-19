---
locale: fr
source_revision: 17f954a2ac80c90ce4a20dd8e36b0981b73e6748171ba829abe55cdfdc69b2b3
translation_provenance: en-US-french-reviewed
api_id: "ScanValue::Null"
---
# ScanValue::Null

Choisir la variante d’entrée SQL Null.

```rust
Null
```

## Paramètres, résultat et utilisation courante

Représente SQL NULL ; scan réinitialise Decimal à un zéro non initialisé et efface NullDecimal. Les variantes sont empruntées par scan et peuvent être réutilisées dans la même exécution du navigateur ; les valeurs fabriquées ou provenant d’une autre exécution sont rejetées.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-null" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
