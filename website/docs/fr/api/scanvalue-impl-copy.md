---
locale: fr
source_revision: 7eeabc8ee1ace4f862b185d7c83f7d0a53594b9ff67ac15d988dde4ffaa12e4b
translation_provenance: en-US-french-reviewed
api_id: "ScanValue::impl Copy"
---
# ScanValue::impl Copy

Utiliser l’implémentation Rust Copy.

```rust
impl Copy for ScanValue
```

## Paramètres, résultat et utilisation courante

Copy duplique cette petite énumération ; les données scalaires empruntées ne sont pas copiées en profondeur. C’est une syntaxe de trait Rust natif ; le navigateur ne l’expose pas comme méthode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-impl-copy" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
