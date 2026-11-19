---
locale: fr
source_revision: 2a36a5c46f57052b68644728359c44f15465f6bfca2c60d1540e40c603cc8c5b
translation_provenance: en-US-french-reviewed
api_id: NullDecimal.valid
---
# NullDecimal.valid

Lire ou modifier la présence sans changer le décimal stocké.

```rust
valid: bool
```

## Utilisation courante et sémantique

Ce champ bool public de Rust correspond à un accesseur et un mutateur boolean. Le passer à false n’efface pas le décimal ; le passer à true ne l’initialise pas. L’API JavaScript rejette les valeurs non booléennes avant toute modification.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-valid" locale="fr" />

L’exemple partagé est exécuté en Rust natif et dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
