---
locale: fr
source_revision: 42bb4c692d19b0113bb004bfa29bdbc283b0bb157f90295cf679a7adf7e3388f
translation_provenance: en-US-french-reviewed
api_id: "Decimal::impl Neg"
---
# Decimal::impl Neg

Utiliser la syntaxe Rust Neg avec emprunt.

```rust
impl Neg for & Decimal
```

## Utilisation courante et points à retenir

Implémenté pour &Decimal, afin que les opérandes restent utilisables. Le - unaire conserve l’échelle et initialise le résultat, y compris le zéro par défaut. Le trait délègue à la méthode nommée correspondante du cœur. Les opérateurs JavaScript du navigateur ne sont pas surchargés ; utilisez add/sub/neg sur l’API JavaScript.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-neg" locale="fr" />

## Exécution

Cet élément relève de la syntaxe des traits Rust natifs ; il n’expose pas d’opérateur JavaScript ou d’objet SQLx du même nom. Le banc d’essai natif par défaut de la documentation exécute cet exemple.

## Méthodes associées

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
