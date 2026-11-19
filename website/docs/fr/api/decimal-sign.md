---
locale: fr
source_revision: eb4bab6a9fdfe771d31eefc8731c88dad81f2f8a3273adea4040fea11f4141ab
translation_provenance: en-US-french-reviewed
api_id: Decimal::sign
---
# Decimal::sign

Lire le signe sous la forme -1, 0 ou 1.

```rust
fn sign (& self) -> i32
```

## Paramètres, résultat et remarques

Renvoie i32/number. Le signe des zéros initialisés et non initialisés est 0 ; aucun signe distinct de zéro négatif n’est conservé.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-sign" locale="fr" />

## Méthodes associées

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
