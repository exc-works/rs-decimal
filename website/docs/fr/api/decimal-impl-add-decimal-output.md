---
locale: fr
source_revision: 7943930854d815d633da7c1bcf125b3fedc2071953907c36e9c6ae78fdf199c7
translation_provenance: en-US-french-reviewed
api_id: "Decimal::impl Add < & Decimal >::Output"
---
# Decimal::impl Add < & Decimal >::Output

Utiliser la syntaxe Rust Add avec emprunt.

```rust
type Output = Decimal
```

## Utilisation courante et points à retenir

Implémenté pour &Decimal, afin que les opérandes restent utilisables. Le résultat exact utilise la plus grande échelle des opérandes et est initialisé. Le type associé Output est Decimal. Les résultats négatifs sont autorisés ; ces opérateurs n’appliquent pas les contraintes de résultat non négatif de safe_add/safe_sub. Le trait délègue à la méthode nommée correspondante du cœur. Les opérateurs JavaScript du navigateur ne sont pas surchargés ; utilisez add/sub/neg sur l’API JavaScript.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-add-decimal-output" locale="fr" />

## Exécution

Cet élément relève de la syntaxe des traits Rust natifs ; il n’expose pas d’opérateur JavaScript ou d’objet SQLx du même nom. Le banc d’essai natif par défaut de la documentation exécute cet exemple.

## Méthodes associées

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
