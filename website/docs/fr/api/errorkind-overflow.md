---
locale: fr
source_revision: b5e19b2f97fd7e9d8a942453bc1d3336bd82a5f0e33e943baa1dc21eb3e5ac5f
translation_provenance: en-US-french-reviewed
api_id: "ErrorKind::Overflow"
---
# ErrorKind::Overflow

Identifier les erreurs Overflow.

```rust
Overflow
```

## Paramètres, résultat et utilisation courante

Un coefficient, une échelle ou une opération dépasse la plage prise en charge par le cœur. Les erreurs de ressources du navigateur sont distinctes. ErrorKind est une catégorie stable, pas un message textuel. Les valeurs de l’énumération du navigateur sont les noms des variantes Rust ; les erreurs construites utilisent matches(), tandis que les exceptions interceptées du bridge exposent un tableau kinds.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-overflow" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
