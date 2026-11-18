---
locale: fr
source_revision: 292844a00b6e84cbd25e16d6e84fa03c5565c434cf287f28e2959e5ca0c46780
translation_provenance: en-US-french-reviewed
api_id: "DecimalError::matches"
---
# DecimalError::matches

Tester l’appartenance à une catégorie.

```rust
fn matches (& self , kind : ErrorKind) -> bool
```

## Paramètres, résultat et utilisation courante

kind : ErrorKind ; renvoie bool/boolean. L’emprunt ne modifie pas l’erreur. Une erreur peut appartenir à plusieurs catégories simultanément. Le constructeur nommé du navigateur crée une valeur Rust libérable avec message, matches(), kinds() et with_kind(). Les exceptions de bibliothèque levées sont des objets Error JavaScript natifs avec category="library" et un tableau kinds ; utilisez alors e.kinds.includes(ErrorKind.Unmarshal). Les erreurs parameter/resource/moved du bridge sont distinctes et ne contiennent pas de catégories de bibliothèque.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-matches" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
