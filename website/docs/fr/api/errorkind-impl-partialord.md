---
locale: fr
source_revision: b6a1d6732343e8fec8b5e4584b880016ecb9ffdc8cf16c927c04637de618fc19
translation_provenance: en-US-french-reviewed
api_id: "ErrorKind::impl PartialOrd"
---
# ErrorKind::impl PartialOrd

Utiliser l’implémentation Rust PartialOrd.

```rust
impl PartialOrd for ErrorKind
```

## Paramètres, résultat et utilisation courante

Ordonne ErrorKind selon l’ordre de déclaration Rust, pas selon la gravité ou l’ordre alphabétique des noms. C’est une syntaxe de trait Rust natif ; le navigateur ne l’expose pas comme méthode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-partialord" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
