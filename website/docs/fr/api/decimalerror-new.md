---
locale: fr
source_revision: 52cfb6e6f9d640e445d0e6b774455d4a9cdc44bf1d1bc94d3efd717749548fc4
translation_provenance: en-US-french-reviewed
api_id: "DecimalError::new"
---
# DecimalError::new

Créer une erreur avec une catégorie.

```rust
fn new (kind : ErrorKind , message : impl Into < String >) -> Self
```

## Paramètres, résultat et utilisation courante

kind : ErrorKind, message : impl `Into<String>`/string ; renvoie une nouvelle DecimalError. Le message est lisible par l’utilisateur et ne détermine pas l’appartenance aux catégories. Le constructeur nommé du navigateur crée une valeur Rust libérable avec message, matches(), kinds() et with_kind(). Les exceptions de bibliothèque levées sont des objets Error JavaScript natifs avec category="library" et un tableau kinds ; utilisez alors e.kinds.includes(ErrorKind.Unmarshal). Les erreurs parameter/resource/moved de l’API JavaScript sont distinctes et ne contiennent pas de catégories de bibliothèque.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-new" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
