---
locale: fr
source_revision: ca5166a5653606ecffcf807579adb23570a7902ad8051649236d87831df9c02a
translation_provenance: en-US-french-reviewed
api_id: "Decimal::impl Encode < '_ , Postgres >"
---
# Decimal::impl Encode < '_ , Postgres >

Utiliser l’intégration native SQLx PostgreSQL NUMERIC.

```rust
impl Encode < '_ , Postgres > for Decimal
```

## Utilisation courante et points à retenir

Exige la fonctionnalité postgres et les types PostgreSQL de SQLx 0.8. Cette implémentation est réservée au natif : le playground du navigateur n’ouvre pas de connexion à une base de données. encode_by_ref(&self, &mut PgArgumentBuffer) renvoie `Result<IsNull, BoxDynError>`. L’emprunt préserve la valeur ; les octets sont ajoutés au tampon d’arguments existant uniquement après un encodage réussi. Même un Decimal non initialisé est encodé en zéro NUMERIC non NULL. Le format binaire utilise des groupes gros-boutistes en base 10000, conserve l’échelle d’affichage et n’utilise aucun intermédiaire float/BigDecimal. Une échelle supérieure à 16383 ou plus de 131072 chiffres entiers est rejeté ; les erreurs d’encodage préservent le tampon de destination.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-encode-postgres" locale="fr" />

## Exécution

Cet élément relève de la syntaxe des traits Rust natifs ; il n’expose pas d’opérateur JavaScript ou d’objet SQLx du même nom. Le banc d’essai natif par défaut de la documentation exécute cet exemple.

## Méthodes associées

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
