---
locale: fr
source_revision: 69334c1aa4ddf9b63780cdd0f7b3c783c6e1646067bf4562ab026efd47279431
translation_provenance: en-US-french-reviewed
api_id: "ScanValue::Bytes"
---
# ScanValue::Bytes

Choisir la variante d’entrée SQL Bytes.

```rust
Bytes (& 'a [u8])
```

## Paramètres, résultat et utilisation courante

Emprunte des octets de texte décimal, pas des données binaires/BSON de transmission. Le navigateur conserve le Uint8Array jusqu’à scan ; les modifications antérieures à scan sont observées. scan ne modifie jamais l’entrée. Un UTF-8 invalide fait échouer l’analyse décimale. Les variantes sont empruntées par scan et peuvent être réutilisées dans la même exécution du navigateur ; les valeurs fabriquées ou provenant d’une autre exécution sont rejetées.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-bytes" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
