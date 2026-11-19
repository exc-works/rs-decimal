---
locale: fr
source_revision: 725ef7f08917b0c306a1f269aaa8e188bc4212046b8491281df04a8f33b0e006
translation_provenance: en-US-french-reviewed
api_id: "DecimalError::impl fmt :: Display::fmt"
---
# DecimalError::impl fmt :: Display::fmt

Utiliser l’implémentation Rust fmt.

```rust
fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result
```

## Paramètres, résultat et utilisation courante

Display écrit uniquement le message. Utilisez format! ou to_string() ; examinez séparément kinds pour un traitement automatique. C’est une syntaxe de trait Rust natif ; le navigateur ne l’expose pas comme méthode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-fmt-display-fmt" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
