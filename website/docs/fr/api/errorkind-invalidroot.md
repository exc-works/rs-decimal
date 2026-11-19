---
locale: fr
source_revision: 5acf1a9800c0d02bd1cb1fa68c2c1f7068c1bb36cf20034e0a1e404758ce2bc4
translation_provenance: en-US-french-reviewed
api_id: "ErrorKind::InvalidRoot"
---
# ErrorKind::InvalidRoot

Identifier les erreurs InvalidRoot.

```rust
InvalidRoot
```

## Paramètres, résultat et utilisation courante

Le degré de la racine n’est pas strictement positif. ErrorKind est une catégorie stable, pas un message textuel. Les valeurs de l’énumération du navigateur sont les noms des variantes Rust ; les erreurs construites utilisent matches(), tandis que les exceptions interceptées du bridge exposent un tableau kinds.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidroot" locale="fr" />

## Méthodes associées

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
