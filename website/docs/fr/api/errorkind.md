---
locale: fr
source_revision: e6b8dfbe3d3140a4697ec5a94f4d508f975bc9c0847e91c70ef29e95978ddcfb
translation_provenance: en-US-french-reviewed
api_id: "ErrorKind"
---
# ErrorKind

Décrire les catégories stables d’erreurs de bibliothèque.

```rust
pub enum ErrorKind
```

## Paramètres, résultat et utilisation courante

L’énumération possède onze variantes et peut être copiée et ordonnée en Rust. ErrorKind dans le navigateur est une table de noms gelée chargée depuis Rust. Pour traiter une erreur précise, testez l’appartenance à une catégorie plutôt que le message. Le constructeur nommé du navigateur crée une valeur Rust libérable avec message, matches(), kinds() et with_kind(). Les exceptions de bibliothèque levées sont des objets Error JavaScript natifs avec category="library" et un tableau kinds ; utilisez alors e.kinds.includes(ErrorKind.Unmarshal). Les erreurs parameter/resource/moved de l’API JavaScript sont distinctes et ne contiennent pas de catégories de bibliothèque.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
