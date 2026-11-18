---
locale: fr
source_revision: 2c125881cbf08b883eeea1f8f7ddb1472b5bb7418b552e6c5a9a55f4ae4adeb9
translation_provenance: en-US-french-reviewed
api_id: "ErrorKind::NegativeRoot"
---
# ErrorKind::NegativeRoot

Identifier les erreurs NegativeRoot.

```rust
NegativeRoot
```

## Paramètres, résultat et utilisation courante

Une racine de degré pair a été demandée pour une valeur négative. ErrorKind est une catégorie stable, pas un message textuel. Les valeurs de l’énumération du navigateur sont les noms des variantes Rust ; les erreurs construites utilisent matches(), tandis que les exceptions interceptées du bridge exposent un tableau kinds.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-negativeroot" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
