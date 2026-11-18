---
locale: fr
source_revision: 45b2b4aa308ba6b40046aead6dddbc34fd9a06d38be18fb996c0bc76cff67656
translation_provenance: en-US-french-reviewed
api_id: Decimal::impl FromStr::Err
---
# Decimal::impl FromStr::Err

Désigner le type d’erreur renvoyé par l’implémentation FromStr de Decimal.

```rust
type Err = DecimalError
```

## Utilisation courante et sémantique

Le type associé `<Decimal as FromStr>::Err` est `DecimalError`. Un texte incorrect renvoie cette erreur sans panic. Consultez ses catégories pour distinguer une syntaxe incorrecte des autres échecs. C’est un alias de type Rust, pas un constructeur JavaScript.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fromstr-err" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
