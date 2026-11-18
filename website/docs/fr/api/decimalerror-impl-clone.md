---
locale: fr
source_revision: 401f5b53ed7c2747c248cad36ae59554a16f09a1f304c6c8d0b65cf689946b2f
translation_provenance: en-US-french-reviewed
api_id: "DecimalError::impl Clone"
---
# DecimalError::impl Clone

Utiliser l’implémentation Rust Clone.

```rust
impl Clone for DecimalError
```

## Paramètres, résultat et utilisation courante

Clone conserve la valeur complète ; les tranches ScanValue empruntées font toujours référence aux données d’origine. C’est une syntaxe de trait Rust natif ; le navigateur ne l’expose pas comme méthode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-clone" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
