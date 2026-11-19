---
locale: fr
source_revision: 68a6c601d819a031d04a43cce835c2569811cfe3ffa0049911e2723c800f6029
translation_provenance: en-US-french-reviewed
api_id: Decimal::r#mod
---
# Decimal::r#mod

Renvoyer le reste signé de la division tronquée.

```rust
fn r#mod (& self , other : & Self) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

other : &Decimal ; Rust renvoie Result<Decimal, DecimalError>. Le reste a le signe du dividende et la plus grande échelle des opérandes ; ce n’est pas un modulo euclidien non négatif. Un diviseur nul renvoie DivideByZero. Les entrées sont empruntées. Rust exige r#mod car mod est un mot-clé ; JavaScript utilise .mod(other). Les deux noms appellent le même comportement du cœur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-r-mod" locale="fr" />

## Méthodes associées

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
