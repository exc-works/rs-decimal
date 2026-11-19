---
locale: fr
source_revision: 51df12e136a0b4ed98ea9f557c4e61c7872827fb92cbe6a97475d66177b43284
translation_provenance: en-US-french-reviewed
api_id: "DecimalError"
---
# DecimalError

Conserver un message et une ou plusieurs catégories d’erreur de bibliothèque.

```rust
pub struct DecimalError
```

## Paramètres, résultat et utilisation courante

DecimalError stocke un message lisible et des catégories ordonnées sans doublons. with_kind consomme la valeur ; matches et kinds l’empruntent. Le type implémente Rust Error et Display. Le constructeur nommé du navigateur crée une valeur Rust libérable avec message, matches(), kinds() et with_kind(). Les exceptions de bibliothèque levées sont des objets Error JavaScript natifs avec category="library" et un tableau kinds ; utilisez alors e.kinds.includes(ErrorKind.Unmarshal). Les erreurs parameter/resource/moved de l’API JavaScript sont distinctes et ne contiennent pas de catégories de bibliothèque.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
