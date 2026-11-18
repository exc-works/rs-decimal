---
locale: fr
source_revision: e8f2b09a741088cc99921865f54ef434d0d70ac376d4e29b7efc3c00a51c3956
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

## Remarques et limites

Ne convertissez pas un number JavaScript en bigint après une perte de précision ; partez d’un littéral entier se terminant par `n` ou d’un texte entier exact. Le playground accepte au maximum 4096 chiffres en entrée ; la bibliothèque native n’est pas soumise à cette politique du playground.

Le navigateur applique également la [politique de ressources](../resource-policy). Les entrées JavaScript sont validées avant l’appel à Rust.

## Méthodes associées

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
