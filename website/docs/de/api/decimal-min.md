---
locale: de
source_revision: eafc21e4a88e29bea4168f39bfe3b5db899f2a28bccbd47f88733ffdaab27a0c
translation_provenance: en-US-german-reviewed
api_id: Decimal::min
---
# Decimal::min

Den kleineren von zwei Zahlenwerten kopieren.

```rust
fn min (& self , other : & Self) -> Self
```

## Parameter, Rückgabe und Hinweise

other: &Decimal ausleihen; liefert einen unabhängigen Decimal. Bei Gleichheit bleiben Skala und Initialisierung des ersten Operanden erhalten. Verwenden Sie in Rust Decimal::min(&left, &right), um nicht die konsumierende Trait-Methode Ord::min auszuwählen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-min" locale="de" />

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
