---
locale: fr
source_revision: bdf3f59a65c991206a82fedc78f9d1dd8953abdf9e7ea6aea954a760e99e1ac6
translation_provenance: en-US-french-reviewed
api_id: "ScanValue::impl Clone"
---
# ScanValue::impl Clone

Utiliser l’implémentation Rust Clone.

```rust
impl Clone for ScanValue
```

## Paramètres, résultat et utilisation courante

Clone conserve la valeur complète ; les tranches ScanValue empruntées font toujours référence aux données d’origine. C’est une syntaxe de trait Rust natif ; le navigateur ne l’expose pas comme méthode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-impl-clone" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
