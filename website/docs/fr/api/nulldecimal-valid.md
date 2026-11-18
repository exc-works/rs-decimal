---
locale: fr
source_revision: d57c1e8ddb07024ed3c5c002f095b2cee086c1a0719df70a0b4ba8761f29418d
translation_provenance: en-US-french-reviewed
api_id: NullDecimal.valid
---
# NullDecimal.valid

Lire ou modifier la présence sans changer le décimal stocké.

```rust
valid: bool
```

## Utilisation courante et sémantique

Ce champ bool public de Rust correspond à un accesseur et un mutateur boolean. Le passer à false n’efface pas le décimal ; le passer à true ne l’initialise pas. Le bridge rejette les valeurs non booléennes avant toute modification.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-valid" locale="fr" />

L’exemple partagé est exécuté en Rust natif et dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
