---
locale: fr
source_revision: 3a7f97312a58229a1540e8d6d6b0b0c1b31b50abeb603d9877b25c182c55eafa
translation_provenance: en-US-french-reviewed
api_id: "ScanValue::impl Copy"
---
# ScanValue::impl Copy

Utiliser l’implémentation Rust Copy.

```rust
impl Copy for ScanValue
```

## Paramètres, résultat et utilisation courante

Copy duplique cette petite énumération ; les données scalaires empruntées ne sont pas copiées en profondeur. C’est une syntaxe de trait Rust natif ; le navigateur ne l’expose pas comme méthode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-impl-copy" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
