---
locale: fr
source_revision: 4b0752061c959ac5ce6e3b7af4bf7c9232d0d1cfb43869372b4caa7a5d6f49f4
translation_provenance: en-US-french-reviewed
api_id: NullDecimal::impl Debug
---
# NullDecimal::impl Debug

Inspecter l’état nullable avec Rust Debug.

```rust
impl Debug for NullDecimal
```

## Utilisation courante et sémantique

Le texte de diagnostic contient la validité et les champs stockés, y compris les valeurs masquées. Ce n’est pas un format de transmission stable ; le navigateur utilise des instantanés.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-impl-debug" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
