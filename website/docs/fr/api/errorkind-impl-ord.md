---
locale: fr
source_revision: 77c1d26d73dd77cab1d0c87534f9e5ec0b5d57b39f76087cd3ef2b288f8c6fae
translation_provenance: en-US-french-reviewed
api_id: "ErrorKind::impl Ord"
---
# ErrorKind::impl Ord

Utiliser l’implémentation Rust Ord.

```rust
impl Ord for ErrorKind
```

## Paramètres, résultat et utilisation courante

Ordonne ErrorKind selon l’ordre de déclaration Rust, pas selon la gravité ou l’ordre alphabétique des noms. C’est une syntaxe de trait Rust natif ; le navigateur ne l’expose pas comme méthode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-ord" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
