---
locale: fr
source_revision: c70b9c986ae1c2d1db6f94784745de5a4d3bb8a7cf3f6af3550e9be48e0f551f
translation_provenance: en-US-french-reviewed
api_id: "DecimalError::new"
---
# DecimalError::new

Créer une erreur avec une catégorie.

```rust
fn new (kind : ErrorKind , message : impl Into < String >) -> Self
```

## Paramètres, résultat et utilisation courante

kind : ErrorKind, message : impl `Into<String>`/string ; renvoie une nouvelle DecimalError. Le message est lisible par l’utilisateur et ne détermine pas l’appartenance aux catégories. Le constructeur nommé du navigateur crée une valeur Rust libérable avec message, matches(), kinds() et with_kind(). Les exceptions de bibliothèque levées sont des objets Error JavaScript natifs avec category="library" et un tableau kinds ; utilisez alors e.kinds.includes(ErrorKind.Unmarshal). Les erreurs parameter/resource/moved du bridge sont distinctes et ne contiennent pas de catégories de bibliothèque.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-new" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
