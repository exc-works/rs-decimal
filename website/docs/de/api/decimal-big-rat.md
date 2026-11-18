---
locale: de
source_revision: b417c8ff7bf7c7086d2d314348cd490ed506bea3f1d87b65cb817d90defb82d7
translation_provenance: en-US-german-reviewed
api_id: Decimal::big_rat
---
# Decimal::big_rat

Den exakten gekürzten rationalen Wert lesen.

```rust
fn big_rat (& self) -> BigRational
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; Rust liefert BigRational. Die Bridge liefert einen unveränderlichen Snapshot {numerator: bigint, denominator: bigint}, normalisiert mit positivem Nenner. Dezimalskala und nachgestellte Nullen bleiben nicht als rationale Metadaten erhalten; null wird 0/1. Die Eingabe wird ausgeliehen und bleibt unverändert.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-big-rat" locale="de" />

## Grenzen im Browser

Ganzzahlen werden nicht über verlustbehaftete JS-Zahlen übertragen. Konvertierungstupel erhalten Rusts Reihenfolge Wert/Kennzeichen; false verspricht kein exaktes Ergebnis. Ausgeliehene Eingaben bleiben unverändert, Konstruktoren initialisieren Ergebnisse. Die [Ressourcenrichtlinie](../resource-policy) begrenzt Ganzzahleingaben, Ausgabeskala und rationales Koeffizientenwachstum getrennt von Bibliotheksfehlern. Rust-Importsyntax ist ausdrücklich als nativ getestet gekennzeichnet.

## Verwandte Methoden

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
