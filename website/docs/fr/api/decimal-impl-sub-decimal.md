---
locale: fr
source_revision: 5c989601e532ba1185143daec0df2f7cc1f95dc1823993238378295a66882fcf
translation_provenance: en-US-french-reviewed
api_id: "Decimal::impl Sub < & Decimal >"
---
# Decimal::impl Sub < & Decimal >

Utiliser la syntaxe Rust Sub avec emprunt.

```rust
impl Sub < & Decimal > for & Decimal
```

## Utilisation courante et points à retenir

Implémenté pour &Decimal, afin que les opérandes restent utilisables. Le résultat exact utilise la plus grande échelle des opérandes et est initialisé. Le type associé Output est Decimal. Les résultats négatifs sont autorisés ; ces opérateurs n’appliquent pas les contraintes de résultat non négatif de safe_add/safe_sub. Le trait délègue à la méthode nommée correspondante du cœur. Les opérateurs JavaScript du navigateur ne sont pas surchargés ; utilisez add/sub/neg sur le bridge.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-sub-decimal" locale="fr" />

## Exécution

Cet élément relève de la syntaxe des traits Rust natifs ; il n’expose pas d’opérateur JavaScript ou d’objet SQLx du même nom. Le banc d’essai natif par défaut de la documentation exécute cet exemple.

## Méthodes associées

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
