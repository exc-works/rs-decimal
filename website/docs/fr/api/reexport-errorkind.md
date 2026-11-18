---
locale: fr
source_revision: 76aa255f8bc8b60ca22e7c6936a048607e56eb1eaa641962626233731b182883
translation_provenance: en-US-french-reviewed
api_id: "reexport::ErrorKind"
---
# reexport::ErrorKind

Importer ErrorKind depuis la racine du crate.

```rust
pub use error::ErrorKind as ErrorKind
```

## Paramètres, résultat et utilisation courante

Utilisez rs_decimal::ErrorKind pour le même élément public. C’est un chemin d’import, pas une implémentation supplémentaire ni une méthode du navigateur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-reexport-errorkind" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
