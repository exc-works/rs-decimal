---
locale: de
source_revision: 3430467a0f3b43857ec837a38512411503e4016266c068e3065f8bd60687547d
translation_provenance: en-US-german-reviewed
api_id: Decimal::try_from_coefficient
---
# Decimal::try_from_coefficient

Einen initialisierten Wert aus einem übernommenen Koeffizienten und einer Skala konstruieren.

```rust
fn try_from_coefficient (coefficient : BigInt , scale : usize) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

coefficient: BigInt; scale: usize (nichtnegative Nachkommastellen). Erhält nachgestellte Nullen und die Skala auch bei null; die Normalisierung des Textparsers wird nicht verwendet. Über isize::MAX liefert Rust Overflow. Der Browser verwendet exaktes bigint und eine ganzzahlige number im wasm32-usize-Bereich. Skalen über i32::MAX behalten Overflow; andere Skalen über 1000 überschreiten das Playground-Budget. JS bigint ist unveränderlich, also kein verschobener Wrapper.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-try-from-coefficient" locale="de" />

## Verwandte Methoden

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
