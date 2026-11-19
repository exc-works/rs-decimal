---
locale: fr
source_revision: a985525078f122e92ef2d70bf9a962a63b82b868ba67925f976407356bf4ff6b
translation_provenance: en-US-french-reviewed
api_id: "DecimalError::kinds"
---
# DecimalError::kinds

Lire toutes les catégories d’erreur.

```rust
fn kinds (& self) -> & [ErrorKind]
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie &[ErrorKind]/un tableau indépendant de noms de variantes. Les catégories sont dédupliquées et triées dans l’ordre de l’énumération Rust, pas dans l’ordre alphabétique. Le constructeur nommé du navigateur crée une valeur Rust libérable avec message, matches(), kinds() et with_kind(). Les exceptions de bibliothèque levées sont des objets Error JavaScript natifs avec category="library" et un tableau kinds ; utilisez alors e.kinds.includes(ErrorKind.Unmarshal). Les erreurs parameter/resource/moved de l’API JavaScript sont distinctes et ne contiennent pas de catégories de bibliothèque.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-kinds" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
