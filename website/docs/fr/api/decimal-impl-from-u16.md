---
locale: fr
source_revision: 75a559c3970bec7bab88ae97c7db504dbf26cffe260da6d7d7bb4f4c6dd32bc5
translation_provenance: en-US-french-reviewed
api_id: "Decimal::impl From < u16 >"
---
# Decimal::impl From < u16 >

Convertir exactement u16 avec Rust From.

```rust
impl From<u16> for Decimal
```

## Utilisation courante et points à retenir

Consomme u16 et renvoie un Decimal initialisé d’échelle zéro. Aucun intermédiaire flottant ni arrondi n’est utilisé ; la conversion est infaillible pour le type entier source. Le zéro est initialisé, contrairement à Decimal::default(). La plage source est celle du type entier Rust ; la largeur de isize/usize dépend de la cible native. Le navigateur utilise des constructeurs nommés explicites ou les variantes signées/non signées de ConfigValue ; la syntaxe Rust From/Into est réservée au natif.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-from-u16" locale="fr" />

## Exécution

Cet élément relève de la syntaxe des traits Rust natifs ; il n’expose pas d’opérateur JavaScript ou d’objet SQLx du même nom. Le banc d’essai natif par défaut de la documentation exécute cet exemple.

## Méthodes associées

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
