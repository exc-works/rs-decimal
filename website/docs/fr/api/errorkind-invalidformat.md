---
locale: fr
source_revision: 2bcf37ee1521720789e7103cecdce4020a906f8f85dfb286e500fd7c881d8638
translation_provenance: en-US-french-reviewed
api_id: "ErrorKind::InvalidFormat"
---
# ErrorKind::InvalidFormat

Identifier les erreurs InvalidFormat.

```rust
InvalidFormat
```

## Paramètres, résultat et utilisation courante

L’entrée n’est pas une représentation décimale finie prise en charge. ErrorKind est une catégorie stable, pas un message textuel. Les valeurs de l’énumération du navigateur sont les noms des variantes Rust ; les erreurs construites utilisent matches(), tandis que les exceptions interceptées du bridge exposent un tableau kinds.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidformat" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
