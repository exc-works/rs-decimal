---
locale: fr
source_revision: 0dd0f243213461bb54e73305b48db01cf1b491f68e6951552c4179ef98edf6bf
translation_provenance: en-US-french-reviewed
api_id: "reexport::DecimalError"
---
# reexport::DecimalError

Importer DecimalError depuis la racine du crate.

```rust
pub use error::DecimalError as DecimalError
```

## Paramètres, résultat et utilisation courante

Utilisez rs_decimal::DecimalError pour le même élément public. C’est un chemin d’import, pas une implémentation supplémentaire ni une méthode du navigateur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-reexport-decimalerror" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
