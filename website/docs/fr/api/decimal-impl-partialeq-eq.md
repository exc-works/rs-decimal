---
locale: fr
source_revision: 1d252fa81f50f2414d333721b6e7df59a5331a4689705908696e9896f6fb4dbf
translation_provenance: en-US-french-reviewed
api_id: Decimal::impl PartialEq::eq
---
# Decimal::impl PartialEq::eq

Comparer l’égalité numérique avec les opérateurs Rust ou PartialEq.

```rust
fn eq (& self , other : & Self) -> bool
```

## Utilisation courante et sémantique

eq emprunte les deux opérandes et renvoie bool. 12.00 == 12.0. JavaScript utilise equal() ; === teste l’identité des objets enveloppes.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialeq-eq" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
