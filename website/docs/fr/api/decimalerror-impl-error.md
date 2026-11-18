---
locale: fr
source_revision: d94c5a1a48ed0fbe522456d88bc4774ce805cace3b32e2a5428e2527dc3be2ce
translation_provenance: en-US-french-reviewed
api_id: "DecimalError::impl Error"
---
# DecimalError::impl Error

Utiliser l’implémentation Rust Error.

```rust
impl Error for DecimalError
```

## Paramètres, résultat et utilisation courante

À utiliser comme Error Rust standard, y compris pour la propagation d’erreurs encapsulées dans Box. Les catégories supplémentaires ne sont pas une chaîne de causes ; source() vaut None. C’est une syntaxe de trait Rust natif ; le navigateur ne l’expose pas comme méthode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-error" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
