---
locale: fr
source_revision: 74688d1ee73b78a25c1792ba73a6a2e013084b0279376cef739a1b5c8cbed64f
translation_provenance: en-US-french-reviewed
api_id: NullDecimal.decimal
---
# NullDecimal.decimal

Inspecter ou remplacer le décimal stocké indépendamment de la validité.

```rust
decimal: Decimal
```

## Utilisation courante et sémantique

Rust expose un champ Decimal mutable. Le navigateur utilise get_decimal_clone() pour une copie indépendante et set_decimal(value) pour consommer la valeur de remplacement. Aucun adaptateur ne modifie valid. Ces adaptateurs explicites évitent de promettre une référence Rust reflétant les modifications en JavaScript.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-decimal" locale="fr" />

L’exemple partagé est exécuté en Rust natif et dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
