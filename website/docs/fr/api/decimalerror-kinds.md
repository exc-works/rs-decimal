---
locale: fr
source_revision: 51be5745f7c3de3ef96d99823418ed54f498b3a9d7e9e49834c1ccd963cd3475
translation_provenance: en-US-french-reviewed
api_id: "DecimalError::kinds"
---
# DecimalError::kinds

Lire toutes les catégories d’erreur.

```rust
fn kinds (& self) -> & [ErrorKind]
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie &[ErrorKind]/un tableau indépendant de noms de variantes. Les catégories sont dédupliquées et triées dans l’ordre de l’énumération Rust, pas dans l’ordre alphabétique. Le constructeur nommé du navigateur crée une valeur Rust libérable avec message, matches(), kinds() et with_kind(). Les exceptions de bibliothèque levées sont des objets Error JavaScript natifs avec category="library" et un tableau kinds ; utilisez alors e.kinds.includes(ErrorKind.Unmarshal). Les erreurs parameter/resource/moved du bridge sont distinctes et ne contiennent pas de catégories de bibliothèque.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-kinds" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
