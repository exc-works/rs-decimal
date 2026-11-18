---
locale: fr
source_revision: 8675e68949343d33893aa6f81cc6a57f32cdafd3aaf80161520178113f46a31d
translation_provenance: en-US-french-reviewed
api_id: NullDecimal
---
# NullDecimal

Représenter un décimal facultatif dont la validité et la valeur stockée sont indépendantes.

```rust
pub struct NullDecimal
```

## Utilisation courante et sémantique

Seul valid détermine la présence. Une valeur nullable invalide peut conserver un décimal non nul ; une valeur nullable valide peut contenir un zéro non initialisé. Le champ decimal public de Rust est exposé en JavaScript par des adaptateurs explicites de copie et de déplacement. La valeur par défaut est absente.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal" locale="fr" />

L’exemple partagé est exécuté en Rust natif et dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
