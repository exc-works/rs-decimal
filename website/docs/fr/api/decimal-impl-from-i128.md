---
locale: fr
source_revision: 2e4990840d666a1ee0727752c12bd5a6b1f796ab2824b1ffd20da5fc2e284755
translation_provenance: en-US-french-reviewed
api_id: "Decimal::impl From < i128 >"
---
# Decimal::impl From < i128 >

Convertir exactement i128 avec Rust From.

```rust
impl From<i128> for Decimal
```

## Utilisation courante et points à retenir

Consomme i128 et renvoie un Decimal initialisé d’échelle zéro. Aucun intermédiaire flottant ni arrondi n’est utilisé ; la conversion est infaillible pour le type entier source. Le zéro est initialisé, contrairement à Decimal::default(). La plage source est celle du type entier Rust ; la largeur de isize/usize dépend de la cible native. Le navigateur utilise des constructeurs nommés explicites ou les variantes signées/non signées de ConfigValue ; la syntaxe Rust From/Into est réservée au natif.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-from-i128" locale="fr" />

## Exécution

Cet élément relève de la syntaxe des traits Rust natifs ; il n’expose pas d’opérateur JavaScript ou d’objet SQLx du même nom. Le banc d’essai natif par défaut de la documentation exécute cet exemple.

## Méthodes associées

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
