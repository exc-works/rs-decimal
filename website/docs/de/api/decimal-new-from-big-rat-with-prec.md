---
locale: de
source_revision: 53d0bced8bfb282a55a5b5ecd0ca2b5866c74eb8bbf117b7537ea9c77cdf1b4d
translation_provenance: en-US-german-reviewed
api_id: Decimal::new_from_big_rat_with_prec
---
# Decimal::new_from_big_rat_with_prec

Eine rationale Zahl auf eine explizite Nachkommastellenzahl runden.

```rust
fn new_from_big_rat_with_prec (value : & BigRational , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

value: &BigRational; prec: nichtnegatives isize; mode: RoundingMode. Die Bridge verwendet ein exaktes bigint-Objekt aus Zähler/Nenner. Anders als exakte Konvertierung sind nicht abbrechende Brüche wie 1/3 erlaubt. Negative Präzision liefert InvalidPrecision; nötige Rundung mit Unnecessary liefert RoundUnnecessary. Ein Nullnenner ist vor Rust ein ungültiger Bridge-Parameter; ein Nullzähler liefert initialisierte Null mit Skala prec.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-big-rat-with-prec" locale="de" />

## Verwandte Methoden

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
