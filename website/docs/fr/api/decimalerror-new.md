---
locale: fr
source_revision: 634fa480d38c38e977bcb656a0df8ea6f905528b83e0983585b20f7e954e4020
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

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
