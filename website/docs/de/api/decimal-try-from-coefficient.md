---
locale: de
source_revision: 34aa17297c2ce6b5b73e997c910f57874da73c70f9cf52cd3fadd2ab710a529d
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

## Grenzen im Browser

Beachten Sie die obigen Besitzregeln. Zugriff nach einem Move liefert einen moved-Fehler; free ist idempotent. Typen und Aliase werden vor dem Move geprüft, aber Rust-Fehler stellen den konsumierten Empfänger nicht wieder her. Die [Ressourcenrichtlinie](../resource-policy) ist von Bibliotheksfehlern getrennt: Potenzen prüfen Zwischenwerte, die Skalenausrichtung prüft das Koeffizientenwachstum. Konservative Prüfungen können auch kleine Endergebnisse ablehnen.

## Verwandte Methoden

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
