---
locale: fr
source_revision: c7a338ce433accfeb61051f993009577dfae7589a629a17bae6043234f27ca38
translation_provenance: en-US-french-reviewed
api_id: Decimal::max
---
# Decimal::max

Copier la plus grande de deux valeurs numériques.

```rust
fn max (& self , other : & Self) -> Self
```

## Paramètres, résultat et remarques

Emprunte other : &Decimal ; renvoie un Decimal indépendant. En cas d’égalité, conserve l’échelle et l’état d’initialisation du premier opérande. En Rust, utilisez Decimal::max(&left, &right) pour sélectionner explicitement cette méthode inhérente avec emprunt ; Ord::max prend possession de la valeur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-max" locale="fr" />

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
