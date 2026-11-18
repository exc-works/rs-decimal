---
locale: de
source_revision: ce7339c2b8115fd42455e27eb624e03a51de3a069de71c01158ea752ddf6833a
translation_provenance: en-US-german-reviewed
api_id: Decimal::bit_len
---
# Decimal::bit_len

Bits des absoluten unskalierten Koeffizienten zählen.

```rust
fn bit_len (& self) -> usize
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; Rust liefert usize, die Bridge number. Die Skala wird nicht angewandt und das Vorzeichenbit nicht gezählt: -1.20 hat Koeffizient -120 und Bitlänge 7. Null einschließlich Standardnull liefert 0. Dies ist nicht der Index des höchsten gesetzten Bits.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-bit-len" locale="de" />

## Grenzen im Browser

Ganzzahlen werden nicht über verlustbehaftete JS-Zahlen übertragen. Konvertierungstupel erhalten Rusts Reihenfolge Wert/Kennzeichen; false verspricht kein exaktes Ergebnis. Ausgeliehene Eingaben bleiben unverändert, Konstruktoren initialisieren Ergebnisse. Die [Ressourcenrichtlinie](../resource-policy) begrenzt Ganzzahleingaben, Ausgabeskala und rationales Koeffizientenwachstum getrennt von Bibliotheksfehlern. Rust-Importsyntax ist ausdrücklich als nativ getestet gekennzeichnet.

## Verwandte Methoden

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
