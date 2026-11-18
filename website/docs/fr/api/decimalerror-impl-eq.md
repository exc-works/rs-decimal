---
locale: fr
source_revision: 45b6d1744039f6508c00288ae029079b73ae286f9378874ea08b456f6c989867
translation_provenance: en-US-french-reviewed
api_id: "DecimalError::impl Eq"
---
# DecimalError::impl Eq

Utiliser l’implémentation Rust Eq.

```rust
impl Eq for DecimalError
```

## Paramètres, résultat et utilisation courante

L’égalité compare le message et les catégories complets de l’erreur, ou la variante exacte de l’énumération. L’égalité de DecimalError est plus stricte que l’appartenance à une catégorie. C’est une syntaxe de trait Rust natif ; le navigateur ne l’expose pas comme méthode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-eq" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
