---
locale: fr
source_revision: 174ec0afec71ca2dec4f0c9c844589a1e01782200d059f787f800837923cca1b
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

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
