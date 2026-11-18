---
locale: fr
source_revision: a35340d9c55b8a08723d8ff0cfa4529bad65cad374d1045d7ce321047fc2a260
translation_provenance: en-US-french-reviewed
api_id: Decimal::new_from_uint64
---
# Decimal::new_from_uint64

Créer un décimal à partir d’un coefficient non signé sur 64 bits. À utiliser pour une source entière non négative dont le nombre de décimales est fourni séparément.

```rust
fn new_from_uint64 (value : u64 , prec : isize) -> Result < Self , DecimalError >
```

## Paramètres et résultat

`value: u64` → `bigint` non négatif ; `prec: isize` → `number` entier. Renvoie `value × 10^-prec`.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-uint64" locale="fr" />

## Remarques et limites

Les valeurs négatives et celles dépassant u64::MAX sont rejetées avant l’entrée dans Rust. Une précision négative produit l’erreur de bibliothèque `InvalidPrecision`. Un number JavaScript ne peut pas transporter fidèlement toute la plage u64.

Le navigateur applique également la [politique de ressources](../resource-policy). Les entrées JavaScript sont validées avant l’appel à Rust.

## Méthodes associées

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
