---
locale: de
source_revision: 91bb241ee917c58402441fd00f901b8aa75a693fa5e9137596c74ca66e946c7e
translation_provenance: en-US-german-reviewed
api_id: Decimal::int64
---
# Decimal::int64

Mit Erfolgskennzeichen in eine Ganzzahl fester Breite konvertieren.

```rust
fn int64 (& self) -> (i64 , bool)
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert (i64, bool), abgebildet auf [bigint, boolean]. Ganzzahlige Werte im Bereich liefern die Ganzzahl und true. Brüche und Überlauf liefern (0, false), keinen abgeschnittenen Wert und keine Ausnahme. Die Skala allein bestimmt Ganzzahligkeit nicht: 12.00 gelingt. Prüfen Sie stets das Kennzeichen, um Fehler von gültiger Null zu unterscheiden.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-int64" locale="de" />

## Grenzen im Browser

Ganzzahlen werden nicht über verlustbehaftete JS-Zahlen übertragen. Konvertierungstupel erhalten Rusts Reihenfolge Wert/Kennzeichen; false verspricht kein exaktes Ergebnis. Ausgeliehene Eingaben bleiben unverändert, Konstruktoren initialisieren Ergebnisse. Die [Ressourcenrichtlinie](../resource-policy) begrenzt Ganzzahleingaben, Ausgabeskala und rationales Koeffizientenwachstum getrennt von Bibliotheksfehlern. Rust-Importsyntax ist ausdrücklich als nativ getestet gekennzeichnet.

## Verwandte Methoden

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
