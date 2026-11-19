---
locale: fr
source_revision: 66f48b07b8a5c080eb29b01047825ba3448ab4e979a526ae96e347e482fef177
translation_provenance: en-US-french-reviewed
api_id: "ScanValue::Text"
---
# ScanValue::Text

Choisir la variante d’entrée SQL Text.

```rust
Text (& 'a str)
```

## Paramètres, résultat et utilisation courante

Emprunte un texte décimal exact. Une seule paire correspondante de guillemets doubles extérieurs est supprimée ; les séquences d’échappement ne sont pas décodées comme du JSON. L’échelle des valeurs numériquement non nulles est conservée après analyse. Les variantes sont empruntées par scan et peuvent être réutilisées dans la même exécution du navigateur ; les valeurs fabriquées ou provenant d’une autre exécution sont rejetées.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-text" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
