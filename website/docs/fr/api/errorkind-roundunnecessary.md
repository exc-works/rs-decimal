---
locale: fr
source_revision: dc7af33554badb1c95cd11064df3b51faeaca3112afc1b82cf2efcaf4ff1844d
translation_provenance: en-US-french-reviewed
api_id: "ErrorKind::RoundUnnecessary"
---
# ErrorKind::RoundUnnecessary

Identifier les erreurs RoundUnnecessary.

```rust
RoundUnnecessary
```

## Paramètres, résultat et utilisation courante

Un arrondi exact a été demandé, mais des chiffres non nuls seraient supprimés. ErrorKind est une catégorie stable, pas un message textuel. Les valeurs de l’énumération du navigateur sont les noms des variantes Rust ; les erreurs construites utilisent matches(), tandis que les exceptions interceptées du bridge exposent un tableau kinds.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-roundunnecessary" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
