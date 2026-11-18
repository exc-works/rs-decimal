---
locale: fr
source_revision: 056afbd5297b8421283bf4270ff73e28ea9129becd2598b31ff7a84ee04dacd5
translation_provenance: en-US-french-reviewed
api_id: NullDecimal::impl Clone
---
# NullDecimal::impl Clone

Cloner en Rust la validité et le décimal stocké.

```rust
impl Clone for NullDecimal
```

## Utilisation courante et sémantique

Le clonage conserve les valeurs masquées même lorsque valid vaut false. Il s’agit d’un trait Rust natif, pas d’une référence partagée qui reflète les modifications.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-impl-clone" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
