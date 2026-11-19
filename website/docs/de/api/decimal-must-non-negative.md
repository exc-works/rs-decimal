---
locale: de
source_revision: 6440a61a644ecdbf15d2da0254f2f46ec6a0bac106e6a427225c0f45a05d250f
translation_provenance: en-US-german-reviewed
api_id: Decimal::must_non_negative
---
# Decimal::must_non_negative

Nichtnegativität verlangen und eine initialisierte Kopie zurückgeben.

```rust
fn must_non_negative (& self) -> Self
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert für nichtnegative Eingaben einschließlich null einen Decimal. Eine negative Eingabe löst in Rust einen Panic mit Zeichenfolgen-Payload aus. In Wasm ist dies ein Trap, kein behandelbarer DecimalError; der Playground-Lauf endet. Ein neuer Lauf erzeugt frischen Wasm-Zustand. Dies ist eine numerische Zusicherung, keine Absolutwertkonvertierung; verwenden Sie abs zum Ändern des Vorzeichens.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-must-non-negative" locale="de" />

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
