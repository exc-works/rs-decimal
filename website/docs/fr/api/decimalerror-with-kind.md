---
locale: fr
source_revision: 3ce42c12330a0659d9f932da918ccfd356a91e7dc3aa65b330f45700001e6ba2
translation_provenance: en-US-french-reviewed
api_id: "DecimalError::with_kind"
---
# DecimalError::with_kind

Consommer une erreur et ajouter une catégorie.

```rust
fn with_kind (mut self , kind : ErrorKind) -> Self
```

## Paramètres, résultat et utilisation courante

kind : ErrorKind ; consomme self et renvoie une nouvelle DecimalError. Les catégories et le message existants sont conservés ; les doublons sont ignorés. Le handle d’origine dans le navigateur devient inutilisable. Les valeurs kind JavaScript invalides sont rejetées avant le déplacement. Le constructeur nommé du navigateur crée une valeur Rust libérable avec message, matches(), kinds() et with_kind(). Les exceptions de bibliothèque levées sont des objets Error JavaScript natifs avec category="library" et un tableau kinds ; utilisez alors e.kinds.includes(ErrorKind.Unmarshal). Les erreurs parameter/resource/moved de l’API JavaScript sont distinctes et ne contiennent pas de catégories de bibliothèque.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-with-kind" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
