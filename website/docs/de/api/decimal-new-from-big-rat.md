---
locale: de
source_revision: c7909226b96ffde911017d8ead0a3f2edd2a0c894ee744bf3bcbea66574f1bf5
translation_provenance: en-US-german-reviewed
api_id: Decimal::new_from_big_rat
---
# Decimal::new_from_big_rat

Eine rationale Zahl in eine exakte endliche Dezimalzahl konvertieren.

```rust
fn new_from_big_rat (value : & BigRational) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

value: &BigRational; die Bridge akzeptiert {numerator: bigint, denominator: bigint}. Der Nenner darf nicht null sein, sonst entsteht ein Bridge-Parameterfehler. Rust BigRational normalisiert Vorzeichen und gemeinsame Faktoren. Nur gekürzte Nenner mit den Primfaktoren 2 und 5 ergeben endliche Dezimalzahlen; andere Faktoren liefern InvalidFormat. Das Ergebnis ist initialisiert. Browserprüfungen greifen nach Kürzung und begrenzen Ausgabeskala/Koeffizientenwachstum vor der Kernkonvertierung.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-big-rat" locale="de" />

## Grenzen im Browser

Ganzzahlen werden nicht über verlustbehaftete JS-Zahlen übertragen. Konvertierungstupel erhalten Rusts Reihenfolge Wert/Kennzeichen; false verspricht kein exaktes Ergebnis. Ausgeliehene Eingaben bleiben unverändert, Konstruktoren initialisieren Ergebnisse. Die [Ressourcenrichtlinie](../resource-policy) begrenzt Ganzzahleingaben, Ausgabeskala und rationales Koeffizientenwachstum getrennt von Bibliotheksfehlern. Rust-Importsyntax ist ausdrücklich als nativ getestet gekennzeichnet.

## Verwandte Methoden

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
