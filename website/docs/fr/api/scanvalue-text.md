---
locale: fr
source_revision: b5a3d9a8d6370fd602bde442c0194ed8033b601f5d907d38c4cf9f51317197c6
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

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
