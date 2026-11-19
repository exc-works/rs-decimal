---
locale: fr
source_revision: b479610db2fc4071c7650b3bc2c08969f87f046ec9147a92c1fba0a74d41a23f
translation_provenance: en-US-french-reviewed
api_id: "DecimalError::impl fmt :: Display"
---
# DecimalError::impl fmt :: Display

Utiliser l’implémentation Rust fmt.

```rust
impl fmt :: Display for DecimalError
```

## Paramètres, résultat et utilisation courante

Display écrit uniquement le message. Utilisez format! ou to_string() ; examinez séparément kinds pour un traitement automatique. C’est une syntaxe de trait Rust natif ; le navigateur ne l’expose pas comme méthode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-fmt-display" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
