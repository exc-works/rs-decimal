---
locale: fr
source_revision: 41da382d7882f55308ed631f052fb5ac78093f335752392d35d023f744890035
translation_provenance: en-US-french-reviewed
api_id: Decimal::new_from_big_int
---
# Decimal::new_from_big_int

Copier un entier de taille arbitraire dans un Decimal initialisé d’échelle zéro, sans perdre de chiffres.

```rust
fn new_from_big_int (value : & BigInt) -> Self
```

## Paramètres et résultat

Rust emprunte `&BigInt` ; JavaScript fournit un `bigint` immuable. Le coefficient renvoyé est indépendant de la source.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-big-int" locale="fr" />

## Conseils d’utilisation

Ne convertissez pas un number JavaScript en bigint après une perte de précision ; partez d’un littéral entier se terminant par `n` ou d’un texte entier exact.

## Méthodes associées

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
