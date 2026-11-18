---
locale: fr
source_revision: 4a909248a2e14bd655a50dd72e826f6d87a3347b735e76ddfe31ae9a4d4b4671
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

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
