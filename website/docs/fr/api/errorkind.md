---
locale: fr
source_revision: c136bea39fff64039ef208c269a0655f2c126075df944740bc4d24f9d953ef55
translation_provenance: en-US-french-reviewed
api_id: "ErrorKind"
---
# ErrorKind

Décrire les catégories stables d’erreurs de bibliothèque.

```rust
pub enum ErrorKind
```

## Paramètres, résultat et utilisation courante

L’énumération possède onze variantes et peut être copiée et ordonnée en Rust. ErrorKind dans le navigateur est une table de noms gelée chargée depuis Rust. Pour traiter une erreur précise, testez l’appartenance à une catégorie plutôt que le message. Le constructeur nommé du navigateur crée une valeur Rust libérable avec message, matches(), kinds() et with_kind(). Les exceptions de bibliothèque levées sont des objets Error JavaScript natifs avec category="library" et un tableau kinds ; utilisez alors e.kinds.includes(ErrorKind.Unmarshal). Les erreurs parameter/resource/moved du bridge sont distinctes et ne contiennent pas de catégories de bibliothèque.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
