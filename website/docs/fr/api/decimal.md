---
locale: fr
source_revision: 56094af9f622985dec933077af65cd0cc673023dc5a3a4be3553f2af47b621d4
translation_provenance: en-US-french-reviewed
api_id: Decimal
---
# Decimal

Stocker un coefficient exact et une échelle décimale non négative.

```rust
pub struct Decimal
```

## Utilisation courante et sémantique

La valeur est coefficient × 10^-scale. L’échelle compte les décimales, y compris les zéros finaux conservés. La valeur par défaut est un zéro numérique non initialisé. Les constructeurs créent des valeurs initialisées ; les méthodes avec emprunt préservent les entrées, tandis que les méthodes qui consomment les valeurs invalident leurs enveloppes dans le navigateur. Utilisez les constructeurs nommés de l’API JavaScript ; new Decimal() n’est pas un constructeur public.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal" locale="fr" />

L’exemple partagé est exécuté en Rust natif et dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
