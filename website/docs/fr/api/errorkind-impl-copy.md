---
locale: fr
source_revision: 473b8cc7f11457cef7cc926925c4f230754afa71c4dea38c85c3b954c8ca89fd
translation_provenance: en-US-french-reviewed
api_id: "ErrorKind::impl Copy"
---
# ErrorKind::impl Copy

Utiliser l’implémentation Rust Copy.

```rust
impl Copy for ErrorKind
```

## Paramètres, résultat et utilisation courante

Copy duplique cette petite énumération ; les données scalaires empruntées ne sont pas copiées en profondeur. C’est une syntaxe de trait Rust natif ; le navigateur ne l’expose pas comme méthode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-copy" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
