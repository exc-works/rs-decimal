---
locale: fr
source_revision: ff9a08f2b3feca78c7b4250dd2c69ee67ba05009b21ca77af28e0824e74ae8d9
translation_provenance: en-US-french-reviewed
api_id: "ScanValue::impl Debug"
---
# ScanValue::impl Debug

Utiliser l’implémentation Rust Debug.

```rust
impl Debug for ScanValue
```

## Paramètres, résultat et utilisation courante

Debug fournit un texte de diagnostic, pas une sérialisation de transmission stable. C’est une syntaxe de trait Rust natif ; le navigateur ne l’expose pas comme méthode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-impl-debug" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
