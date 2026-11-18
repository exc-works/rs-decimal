---
locale: fr
source_revision: 9459cb9d77c7f6ad22b96853baef01b1a9461529cecff18a116b5d2dee9166d3
translation_provenance: en-US-french-reviewed
api_id: Decimal
---
# Decimal

Stocker un coefficient exact et une échelle décimale non négative.

```rust
pub struct Decimal
```

## Utilisation courante et sémantique

La valeur est coefficient × 10^-scale. L’échelle compte les décimales, y compris les zéros finaux conservés. La valeur par défaut est un zéro numérique non initialisé. Les constructeurs créent des valeurs initialisées ; les méthodes avec emprunt préservent les entrées, tandis que les méthodes qui consomment les valeurs invalident leurs enveloppes dans le navigateur. Utilisez les constructeurs nommés du bridge ; new Decimal() n’est pas un constructeur public.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal" locale="fr" />

L’exemple partagé est exécuté en Rust natif et dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
