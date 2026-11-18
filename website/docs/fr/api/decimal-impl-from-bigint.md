---
locale: fr
source_revision: 7fadd186172ed1d2675ccc16ae4020af71fd55a578a479ca70e021285e76f370
translation_provenance: en-US-french-reviewed
api_id: "Decimal::impl From < BigInt >"
---
# Decimal::impl From < BigInt >

Convertir exactement BigInt avec Rust From.

```rust
impl From < BigInt > for Decimal
```

## Utilisation courante et points à retenir

Consomme BigInt et renvoie un Decimal initialisé d’échelle zéro. Aucun intermédiaire flottant ni arrondi n’est utilisé ; la conversion est infaillible pour le type entier source. Le zéro est initialisé, contrairement à Decimal::default(). La propriété du BigInt est transférée au décimal. Le navigateur utilise des constructeurs nommés explicites ou les variantes signées/non signées de ConfigValue ; la syntaxe Rust From/Into est réservée au natif.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-from-bigint" locale="fr" />

## Exécution

Cet élément relève de la syntaxe des traits Rust natifs ; il n’expose pas d’opérateur JavaScript ou d’objet SQLx du même nom. Le banc d’essai natif par défaut de la documentation exécute cet exemple.

## Méthodes associées

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
