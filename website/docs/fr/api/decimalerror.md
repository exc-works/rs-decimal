---
locale: fr
source_revision: 8805baa34ac15d1bc6208bedc88ae9d1d05c8235150f9f2331a9105561b652f3
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

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
