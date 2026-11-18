---
locale: fr
source_revision: 7dc5d9b6c0f5030d430dc8fa4c821516f6b5bfe8979234b7de2b0cf0b1d59588
translation_provenance: en-US-french-reviewed
api_id: "Decimal::impl Type < Postgres >"
---
# Decimal::impl Type < Postgres >

Utiliser l’intégration native SQLx PostgreSQL NUMERIC.

```rust
impl Type < Postgres > for Decimal
```

## Utilisation courante et points à retenir

Exige la fonctionnalité postgres et les types PostgreSQL de SQLx 0.8. Cette implémentation est réservée au natif : le playground du navigateur n’ouvre pas de connexion à une base de données. type_info() renvoie NUMERIC OID 1700. compatible(ty) accepte cet OID exact, pas float8, les entiers ou des types textuels arbitraires ; aucune conversion flottante implicite n’est promise.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-type-postgres" locale="fr" />

## Exécution

Cet élément relève de la syntaxe des traits Rust natifs ; il n’expose pas d’opérateur JavaScript ou d’objet SQLx du même nom. Le banc d’essai natif par défaut de la documentation exécute cet exemple.

## Méthodes associées

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
