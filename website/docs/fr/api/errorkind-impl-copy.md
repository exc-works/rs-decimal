---
locale: fr
source_revision: a66891e5a69e79ad1f199a0dd34da946899f3deff1662608f76ae3c569145632
translation_provenance: en-US-french-reviewed
api_id: "ErrorKind::impl Copy"
---
# ErrorKind::impl Copy

Utiliser l’implémentation Rust Copy.

```rust
impl Copy for ErrorKind
```

## Paramètres, résultat et utilisation courante

Copy duplique cette petite énumération ; les données scalaires empruntées ne sont pas copiées en profondeur. C’est une syntaxe de trait Rust natif ; le navigateur ne l’expose pas comme méthode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-copy" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
