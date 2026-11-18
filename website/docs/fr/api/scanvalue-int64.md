---
locale: fr
source_revision: 36f5ff0720c14718444a86498f3f5cd1d16ba0df1d4c60c2d59357d8e469675e
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

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
