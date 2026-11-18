---
locale: fr
source_revision: 21da10a6e76a84c5146b6a083d73a886edce7505fb47206e66d109e57f693aeb
translation_provenance: en-US-french-reviewed
api_id: "Decimal::impl Decode < 'r , Postgres >::decode"
---
# Decimal::impl Decode < 'r , Postgres >::decode

Utiliser l’intégration native SQLx PostgreSQL NUMERIC.

```rust
fn decode (value : PgValueRef < 'r >) -> Result < Self , BoxDynError >
```

## Utilisation courante et points à retenir

Exige la fonctionnalité postgres et les types PostgreSQL de SQLx 0.8. Cette implémentation est réservée au natif : le playground du navigateur n’ouvre pas de connexion à une base de données. decode(PgValueRef) renvoie `Result<Self, BoxDynError>`. SQLx fournit la valeur empruntée à la base de données ; ne fabriquez pas sa représentation de transmission privée. NUMERIC binaire et textuel conservent la valeur et l’échelle d’affichage, y compris un zéro avec échelle. SQL NULL est rejeté ; utilisez NullDecimal ou `Option<Decimal>` pour les colonnes nullables. Les résultats autres que null sont initialisés. Les NUMERIC incorrects ou non finis sont rejetés sans arrondi. Cet exemple s’exécute contre un serveur PostgreSQL dédié dans la CI ; aucune base de données externe n’est nécessaire au navigateur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-decode-r-postgres-decode" locale="fr" />

## Exécution

Cet élément relève de la syntaxe des traits Rust natifs ; il n’expose pas d’opérateur JavaScript ou d’objet SQLx du même nom. Exécutez cet exemple avec un serveur PostgreSQL dédié, DATABASE_URL et `cargo run --locked -p decimal-web-examples --features postgres-live`. Sans ce profil, il est explicitement signalé comme ignoré et n’est pas compté comme un contrôle de base de données réussi.

## Méthodes associées

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
