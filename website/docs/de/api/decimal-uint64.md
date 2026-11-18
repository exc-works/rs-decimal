---
locale: de
source_revision: 309b64e5487e4ac1d2ea8cf88852216653b617cb1ed53a7d6bdd1a0dffa2d494
translation_provenance: en-US-german-reviewed
api_id: Decimal::uint64
---
# Decimal::uint64

Mit Erfolgskennzeichen in eine Ganzzahl fester Breite konvertieren.

```rust
fn uint64 (& self) -> (u64 , bool)
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert (u64, bool), abgebildet auf [bigint, boolean]. Ganzzahlige Werte im Bereich liefern die Ganzzahl und true. Brüche und Überlauf liefern (0, false), keinen abgeschnittenen Wert und keine Ausnahme. Negative Werte liefern ebenfalls (0, false). Die Skala allein bestimmt Ganzzahligkeit nicht: 12.00 gelingt. Prüfen Sie stets das Kennzeichen, um Fehler von gültiger Null zu unterscheiden.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-uint64" locale="de" />

## Grenzen im Browser

Ganzzahlen werden nicht über verlustbehaftete JS-Zahlen übertragen. Konvertierungstupel erhalten Rusts Reihenfolge Wert/Kennzeichen; false verspricht kein exaktes Ergebnis. Ausgeliehene Eingaben bleiben unverändert, Konstruktoren initialisieren Ergebnisse. Die [Ressourcenrichtlinie](../resource-policy) begrenzt Ganzzahleingaben, Ausgabeskala und rationales Koeffizientenwachstum getrennt von Bibliotheksfehlern. Rust-Importsyntax ist ausdrücklich als nativ getestet gekennzeichnet.

## Verwandte Methoden

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
