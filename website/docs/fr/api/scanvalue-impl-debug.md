---
locale: fr
source_revision: 9436e2c2329251e2adecf17884000bc9408c12b9f0b7fd847a1b861820048eb7
translation_provenance: en-US-french-reviewed
api_id: "ScanValue::impl Debug"
---
# ScanValue::impl Debug

Utiliser l’implémentation Rust Debug.

```rust
impl Debug for ScanValue
```

## Paramètres, résultat et utilisation courante

Debug fournit un texte de diagnostic, pas une sérialisation de transmission stable. C’est une syntaxe de trait Rust natif ; le navigateur ne l’expose pas comme méthode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-impl-debug" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
