---
locale: fr
source_revision: 65703feb0e11af9dfe348c058c514dbf10ab13c1d2a720632926e57fee628fa9
translation_provenance: en-US-french-reviewed
api_id: Decimal::impl FromStr
---
# Decimal::impl FromStr

Analyser un texte décimal avec Rust parse et FromStr.

```rust
impl FromStr for Decimal
```

## Utilisation courante et sémantique

Renvoie Decimal ou DecimalError. La syntaxe et la normalisation du zéro sont identiques à new_from_string. Le navigateur appelle cette méthode nommée ; la syntaxe du trait Rust est uniquement native.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fromstr" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
