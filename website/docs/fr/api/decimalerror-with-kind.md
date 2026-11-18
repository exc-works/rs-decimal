---
locale: fr
source_revision: f5327cbdbb40212ac4046096b90742ab088011d23930faef1e12c326d8e03e09
translation_provenance: en-US-french-reviewed
api_id: "DecimalError::with_kind"
---
# DecimalError::with_kind

Consommer une erreur et ajouter une catégorie.

```rust
fn with_kind (mut self , kind : ErrorKind) -> Self
```

## Paramètres, résultat et utilisation courante

kind : ErrorKind ; consomme self et renvoie une nouvelle DecimalError. Les catégories et le message existants sont conservés ; les doublons sont ignorés. Le handle d’origine dans le navigateur devient inutilisable. Les valeurs kind JavaScript invalides sont rejetées avant le déplacement. Le constructeur nommé du navigateur crée une valeur Rust libérable avec message, matches(), kinds() et with_kind(). Les exceptions de bibliothèque levées sont des objets Error JavaScript natifs avec category="library" et un tableau kinds ; utilisez alors e.kinds.includes(ErrorKind.Unmarshal). Les erreurs parameter/resource/moved du bridge sont distinctes et ne contiennent pas de catégories de bibliothèque.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-with-kind" locale="fr" />

## Limites du navigateur

Les entrées SQL texte/octets et les messages d’erreur sont limités à 8192 octets UTF-8. Le texte SQL est soumis au [contrôle des ressources](../resource-policy) après retrait des guillemets extérieurs : ceux-ci ne peuvent donc pas contourner les limites d’exposant. Les appels avec emprunt préservent les entrées ; les handles consommés ou libérés ne peuvent pas être réutilisés. Les exemples de traits et d’imports Rust s’exécutent dans les tests natifs et ne prétendent pas fournir des méthodes du navigateur.

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
