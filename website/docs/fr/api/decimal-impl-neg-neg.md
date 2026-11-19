---
locale: fr
source_revision: ae30eee52b8243c53d2363e82db8a8847f306fed0233c1fadf3d83f102fe5452
translation_provenance: en-US-french-reviewed
api_id: "Decimal::impl Neg::neg"
---
# Decimal::impl Neg::neg

Utiliser la syntaxe Rust Neg avec emprunt.

```rust
fn neg (self) -> Decimal
```

## Utilisation courante et points à retenir

Implémenté pour &Decimal, afin que les opérandes restent utilisables. Le - unaire conserve l’échelle et initialise le résultat, y compris le zéro par défaut. Le trait délègue à la méthode nommée correspondante du cœur. Les opérateurs JavaScript du navigateur ne sont pas surchargés ; utilisez add/sub/neg sur l’API JavaScript.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-neg-neg" locale="fr" />

## Exécution

Cet élément relève de la syntaxe des traits Rust natifs ; il n’expose pas d’opérateur JavaScript ou d’objet SQLx du même nom. Le banc d’essai natif par défaut de la documentation exécute cet exemple.

## Méthodes associées

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
