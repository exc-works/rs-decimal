---
locale: fr
source_revision: 1b410a44c40ddf1ed7ab08ecafd94bf955a8c6e8ace55716beba7530f0db9122
translation_provenance: en-US-french-reviewed
api_id: "ErrorKind::impl Debug"
---
# ErrorKind::impl Debug

Utiliser l’implémentation Rust Debug.

```rust
impl Debug for ErrorKind
```

## Paramètres, résultat et utilisation courante

Debug fournit un texte de diagnostic, pas une sérialisation de transmission stable. C’est une syntaxe de trait Rust natif ; le navigateur ne l’expose pas comme méthode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-debug" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
