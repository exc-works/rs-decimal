---
locale: fr
source_revision: 3bfbb5926961d9d892e3da2738bc0fc6133b23f056d8610803f8a02d24a533d4
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

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
