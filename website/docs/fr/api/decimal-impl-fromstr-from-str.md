---
locale: fr
source_revision: b8a64fe28faf9f699b61b2f86e331fd9c5b8333882ce85a4476ea07f56d57e1d
translation_provenance: en-US-french-reviewed
api_id: Decimal::impl FromStr::from_str
---
# Decimal::impl FromStr::from_str

Analyser un texte décimal avec Rust parse et FromStr.

```rust
fn from_str (text : & str) -> Result < Self , Self :: Err >
```

## Utilisation courante et sémantique

Renvoie Decimal ou DecimalError. La syntaxe et la normalisation du zéro sont identiques à new_from_string. Le navigateur appelle cette méthode nommée ; la syntaxe du trait Rust est uniquement native.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fromstr-from-str" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
