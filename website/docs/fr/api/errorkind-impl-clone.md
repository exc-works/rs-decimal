---
locale: fr
source_revision: b4b7e965750a7655fdeb57385feedb3c62874c4ab66fdfb6a3f2062d579960d6
translation_provenance: en-US-french-reviewed
api_id: "ErrorKind::impl Clone"
---
# ErrorKind::impl Clone

Utiliser l’implémentation Rust Clone.

```rust
impl Clone for ErrorKind
```

## Paramètres, résultat et utilisation courante

Clone conserve la valeur complète ; les tranches ScanValue empruntées font toujours référence aux données d’origine. C’est une syntaxe de trait Rust natif ; le navigateur ne l’expose pas comme méthode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-clone" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
