---
locale: fr
source_revision: 61f8c035d4ee9a0675e175237d6b888b80543e4b200bab686287d3d96e9f1847
translation_provenance: en-US-french-reviewed
api_id: "Decimal::impl From < u8 >"
---
# Decimal::impl From < u8 >

Convertir exactement u8 avec Rust From.

```rust
impl From<u8> for Decimal
```

## Utilisation courante et points à retenir

Consomme u8 et renvoie un Decimal initialisé d’échelle zéro. Aucun intermédiaire flottant ni arrondi n’est utilisé ; la conversion est infaillible pour le type entier source. Le zéro est initialisé, contrairement à Decimal::default(). La plage source est celle du type entier Rust ; la largeur de isize/usize dépend de la cible native. Le navigateur utilise des constructeurs nommés explicites ou les variantes signées/non signées de ConfigValue ; la syntaxe Rust From/Into est réservée au natif.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-from-u8" locale="fr" />

## Exécution

Cet élément relève de la syntaxe des traits Rust natifs ; il n’expose pas d’opérateur JavaScript ou d’objet SQLx du même nom. Le banc d’essai natif par défaut de la documentation exécute cet exemple.

## Méthodes associées

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
