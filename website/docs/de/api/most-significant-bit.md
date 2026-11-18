---
locale: de
source_revision: cbc68f86b580dd7b41c6b741193a12b44e7d04cdf771d632d510adf82d84c1ba
translation_provenance: en-US-german-reviewed
api_id: most_significant_bit
---
# most_significant_bit

Den nullbasierten Index des höchsten gesetzten Bits ermitteln.

```rust
fn most_significant_bit (value : & BigInt) -> Result < usize , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

value: &BigInt, dargestellt als bigint. Liefert Result<usize, DecimalError>/number. Null und eins liefern beide 0; 120 liefert 6. Negative Eingabe liefert InvalidArgument. Dies ist eine freie Funktion, keine Decimal-Methode; übergeben Sie coefficient(), wenn dessen Ganzzahl gemeint ist.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-most-significant-bit" locale="de" />

## Grenzen im Browser

Ganzzahlen werden nicht über verlustbehaftete JS-Zahlen übertragen. Konvertierungstupel erhalten Rusts Reihenfolge Wert/Kennzeichen; false verspricht kein exaktes Ergebnis. Ausgeliehene Eingaben bleiben unverändert, Konstruktoren initialisieren Ergebnisse. Die [Ressourcenrichtlinie](../resource-policy) begrenzt Ganzzahleingaben, Ausgabeskala und rationales Koeffizientenwachstum getrennt von Bibliotheksfehlern. Rust-Importsyntax ist ausdrücklich als nativ getestet gekennzeichnet.

## Verwandte Methoden

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
