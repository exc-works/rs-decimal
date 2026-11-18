---
locale: fr
source_revision: e7411932b8884630967e9c3e34bf9db436dc8920df8139ae778ac83fc80fdcb3
translation_provenance: en-US-french-reviewed
api_id: "DecimalError"
---
# DecimalError

Conserver un message et une ou plusieurs catégories d’erreur de bibliothèque.

```rust
pub struct DecimalError
```

## Paramètres, résultat et utilisation courante

DecimalError stocke un message lisible et des catégories ordonnées sans doublons. with_kind consomme la valeur ; matches et kinds l’empruntent. Le type implémente Rust Error et Display. Le constructeur nommé du navigateur crée une valeur Rust libérable avec message, matches(), kinds() et with_kind(). Les exceptions de bibliothèque levées sont des objets Error JavaScript natifs avec category="library" et un tableau kinds ; utilisez alors e.kinds.includes(ErrorKind.Unmarshal). Les erreurs parameter/resource/moved du bridge sont distinctes et ne contiennent pas de catégories de bibliothèque.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
