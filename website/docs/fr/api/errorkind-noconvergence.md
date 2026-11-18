---
locale: fr
source_revision: 7a9119eb74e99c5e35f09e87df2556b438ba2f0ad510d5e668cdcc13dc40e550
translation_provenance: en-US-french-reviewed
api_id: "ErrorKind::NoConvergence"
---
# ErrorKind::NoConvergence

Identifier les erreurs NoConvergence.

```rust
NoConvergence
```

## Paramètres, résultat et utilisation courante

L’algorithme borné n’a pas pu certifier un résultat arrondi ; cela ne signifie pas que le résultat mathématique n’existe pas. ErrorKind est une catégorie stable, pas un message textuel. Les valeurs de l’énumération du navigateur sont les noms des variantes Rust ; les erreurs construites utilisent matches(), tandis que les exceptions interceptées du bridge exposent un tableau kinds.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-noconvergence" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
