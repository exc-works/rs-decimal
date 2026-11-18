---
locale: fr
source_revision: d175f05e819f4b3e7b18eefbd00d8a57a38edc9a97413b4a95e100580b76c12b
translation_provenance: en-US-french-reviewed
api_id: "ErrorKind::impl Eq"
---
# ErrorKind::impl Eq

Utiliser l’implémentation Rust Eq.

```rust
impl Eq for ErrorKind
```

## Paramètres, résultat et utilisation courante

L’égalité compare le message et les catégories complets de l’erreur, ou la variante exacte de l’énumération. L’égalité de DecimalError est plus stricte que l’appartenance à une catégorie. C’est une syntaxe de trait Rust natif ; le navigateur ne l’expose pas comme méthode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-eq" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
