---
locale: fr
source_revision: f24ab15a12ac3cc5edffa3bb6354648be7a4944a94929ac82037f54fe89c586d
translation_provenance: en-US-french-reviewed
api_id: "NullDecimal::impl Encode < '_ , Postgres >::encode_by_ref"
---
# NullDecimal::impl Encode < '_ , Postgres >::encode_by_ref

Utiliser l’intégration native SQLx PostgreSQL NUMERIC.

```rust
fn encode_by_ref (& self , buf : & mut PgArgumentBuffer) -> Result < IsNull , BoxDynError >
```

## Utilisation courante et points à retenir

Exige la fonctionnalité postgres et les types PostgreSQL de SQLx 0.8. Cette implémentation est réservée au natif : le playground du navigateur n’ouvre pas de connexion à une base de données. encode_by_ref(&self, &mut PgArgumentBuffer) renvoie `Result<IsNull, BoxDynError>`. L’emprunt préserve la valeur ; les octets sont ajoutés au tampon d’arguments existant uniquement après un encodage réussi. Les valeurs nullables invalides renvoient IsNull::Yes sans octets. Les valeurs valides encodent le Decimal interne, même si son zéro n’était pas initialisé. Le format binaire utilise des groupes gros-boutistes en base 10000, conserve l’échelle d’affichage et n’utilise aucun intermédiaire float/BigDecimal. Une échelle supérieure à 16383 ou plus de 131072 chiffres entiers est rejeté ; les erreurs d’encodage préservent le tampon de destination.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-nulldecimal-impl-encode-postgres-encode-by-ref" locale="fr" />

## Exécution

Cet élément relève de la syntaxe des traits Rust natifs ; il n’expose pas d’opérateur JavaScript ou d’objet SQLx du même nom. Le banc d’essai natif par défaut de la documentation exécute cet exemple.

## Méthodes associées

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
