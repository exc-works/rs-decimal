---
locale: fr
source_revision: 3ced5d99beade676205267c80e93f90c49f974486393990035a12b18f4061c4a
translation_provenance: en-US-french-reviewed
api_id: Decimal::impl PartialEq
---
# Decimal::impl PartialEq

Comparer l’égalité numérique avec les opérateurs Rust ou PartialEq.

```rust
impl PartialEq for Decimal
```

## Utilisation courante et sémantique

eq emprunte les deux opérandes et renvoie bool. 12.00 == 12.0. JavaScript utilise equal() ; === teste l’identité des objets enveloppes.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialeq" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
