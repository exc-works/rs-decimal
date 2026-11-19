---
locale: de
source_revision: 3d99b7dec25de936ed85161e3eb8cf4565f56b88649e6c879a6468d051308404
translation_provenance: en-US-german-reviewed
api_id: Decimal::new_from_big_rat_with_prec
---
# Decimal::new_from_big_rat_with_prec

Eine rationale Zahl auf eine explizite Nachkommastellenzahl runden.

```rust
fn new_from_big_rat_with_prec (value : & BigRational , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

value: &BigRational; prec: nichtnegatives isize; mode: RoundingMode. Die JavaScript-API verwendet ein exaktes bigint-Objekt aus Zähler/Nenner. Anders als exakte Konvertierung sind nicht abbrechende Brüche wie 1/3 erlaubt. Negative Präzision liefert InvalidPrecision; nötige Rundung mit Unnecessary liefert RoundUnnecessary. Ein Nullnenner ist vor Rust ein ungültiger JavaScript-Parameter; ein Nullzähler liefert initialisierte Null mit Skala prec.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-big-rat-with-prec" locale="de" />

## Verwandte Methoden

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
