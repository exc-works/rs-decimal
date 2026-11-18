---
locale: fr
source_revision: 329a8011aecf030884cb84a14d70c08abd0f17ed41927f957f56a638648169ac
translation_provenance: en-US-french-reviewed
api_id: "NullDecimal::impl PgHasArrayType"
---
# NullDecimal::impl PgHasArrayType

Utiliser l’intégration native SQLx PostgreSQL NUMERIC.

```rust
impl PgHasArrayType for NullDecimal
```

## Utilisation courante et points à retenir

Exige la fonctionnalité postgres et les types PostgreSQL de SQLx 0.8. Cette implémentation est réservée au natif : le playground du navigateur n’ouvre pas de connexion à une base de données. array_type_info() renvoie NUMERIC[] OID 1231, permettant les liaisons de tableaux SQLx. Les éléments NullDecimal des tableaux conservent SQL NULL via leur validité ; utilisez des requêtes SQLx natives pour encoder/décoder des tableaux complets.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-nulldecimal-impl-pghasarraytype" locale="fr" />

## Exécution

Cet élément relève de la syntaxe des traits Rust natifs ; il n’expose pas d’opérateur JavaScript ou d’objet SQLx du même nom. Le banc d’essai natif par défaut de la documentation exécute cet exemple.

## Méthodes associées

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
