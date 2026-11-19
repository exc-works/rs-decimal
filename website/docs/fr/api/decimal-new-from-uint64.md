---
locale: fr
source_revision: c4fc0f977bb46b55b53465df0296b049aa252fbfa98d47634f13b46fd3108bd0
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

## Conseils d’utilisation

Les valeurs négatives et celles dépassant u64::MAX sont rejetées avant l’entrée dans Rust. Une précision négative produit l’erreur de bibliothèque `InvalidPrecision`. Un number JavaScript ne peut pas transporter fidèlement toute la plage u64.

## Méthodes associées

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
