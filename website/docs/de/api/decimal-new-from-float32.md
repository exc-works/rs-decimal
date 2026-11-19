---
locale: de
source_revision: c40a135ffb56f404451bd244cc0aedf0eee2b3b5503190541e81212c60c34237
translation_provenance: en-US-german-reviewed
api_id: Decimal::new_from_float32
---
# Decimal::new_from_float32

Aus der kürzesten Dezimaldarstellung konstruieren, die wieder denselben Gleitkommawert ergibt.

```rust
fn new_from_float32 (value : f32) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

value: f32, als JS number übergeben. Die JavaScript-API verengt zunächst binary64 auf binary32; 16777217 wird dadurch 16777216. Bereits durch binäre Gleitkommadarstellung verlorene Dezimalziffern werden nicht wiederhergestellt. Verwenden Sie new_from_string für exakte Dezimaleingaben. NaN und beide Unendlichkeiten liefern InvalidFormat. Ein endliches f64, das bei Verengung auf f32 überläuft, liefert ebenfalls InvalidFormat. Negative Null wird zur initialisierten Decimal-Null ohne Vorzeichen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-float32" locale="de" />

## Verwandte Methoden

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
