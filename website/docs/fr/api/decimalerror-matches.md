---
locale: fr
source_revision: 51d8eb06be2cf1b2c76ae713dc34443717bd6adc00c52dec95e805914eed6ec1
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

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
