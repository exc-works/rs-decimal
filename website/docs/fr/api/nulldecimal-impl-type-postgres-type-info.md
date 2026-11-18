---
locale: fr
source_revision: f56da33c81a26f95ee4ac8cfce759062606ade6eed7055366c2fe672d142ca01
translation_provenance: en-US-french-reviewed
api_id: "NullDecimal::impl Type < Postgres >::type_info"
---
# NullDecimal::impl Type < Postgres >::type_info

Utiliser l’intégration native SQLx PostgreSQL NUMERIC.

```rust
fn type_info () -> PgTypeInfo
```

## Utilisation courante et points à retenir

Exige la fonctionnalité postgres et les types PostgreSQL de SQLx 0.8. Cette implémentation est réservée au natif : le playground du navigateur n’ouvre pas de connexion à une base de données. type_info() renvoie NUMERIC OID 1700. compatible(ty) accepte cet OID exact, pas float8, les entiers ou des types textuels arbitraires ; aucune conversion flottante implicite n’est promise.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-nulldecimal-impl-type-postgres-type-info" locale="fr" />

## Exécution

Cet élément relève de la syntaxe des traits Rust natifs ; il n’expose pas d’opérateur JavaScript ou d’objet SQLx du même nom. Le banc d’essai natif par défaut de la documentation exécute cet exemple.

## Méthodes associées

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
