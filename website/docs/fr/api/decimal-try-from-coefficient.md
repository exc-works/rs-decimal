---
locale: fr
source_revision: 3430467a0f3b43857ec837a38512411503e4016266c068e3065f8bd60687547d
translation_provenance: en-US-french-reviewed
api_id: Decimal::try_from_coefficient
---
# Decimal::try_from_coefficient

Construire une valeur initialisée à partir d’un coefficient possédé et d’une échelle.

```rust
fn try_from_coefficient (coefficient : BigInt , scale : usize) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

coefficient : BigInt ; scale : usize (nombre non négatif de décimales). Conserve les zéros finaux et l’échelle du zéro ; n’utilise pas la normalisation de l’analyseur de texte. Rust renvoie Overflow au-delà de isize::MAX. Le navigateur utilise un bigint exact et un number entier dans la plage usize de wasm32 ; les échelles au-delà de i32::MAX conservent Overflow, et les autres échelles dépassant 1000 excèdent le budget de ressources du playground. Le bigint JavaScript est immuable : ce n’est donc pas un objet enveloppe déplacé.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-try-from-coefficient" locale="fr" />

## Méthodes associées

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
