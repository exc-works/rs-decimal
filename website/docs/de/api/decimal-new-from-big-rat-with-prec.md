---
locale: de
source_revision: 75750ddb09ce29d94b77a840b238d39bf569cfb662c01d997d14bb79d57564c3
translation_provenance: en-US-german-reviewed
api_id: Decimal::new_from_big_rat_with_prec
---
# Decimal::new_from_big_rat_with_prec

Eine rationale Zahl auf eine explizite Nachkommastellenzahl runden.

```rust
fn new_from_big_rat_with_prec (value : & BigRational , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

value: &BigRational; prec: nichtnegatives isize; mode: RoundingMode. Die Bridge verwendet ein exaktes bigint-Objekt aus Zähler/Nenner. Anders als exakte Konvertierung sind nicht abbrechende Brüche wie 1/3 erlaubt. Negative Präzision liefert InvalidPrecision; nötige Rundung mit Unnecessary liefert RoundUnnecessary. Ein Nullnenner ist vor Rust ein ungültiger Bridge-Parameter; ein Nullzähler liefert initialisierte Null mit Skala prec.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-big-rat-with-prec" locale="de" />

## Grenzen im Browser

Ganzzahlen werden nicht über verlustbehaftete JS-Zahlen übertragen. Konvertierungstupel erhalten Rusts Reihenfolge Wert/Kennzeichen; false verspricht kein exaktes Ergebnis. Ausgeliehene Eingaben bleiben unverändert, Konstruktoren initialisieren Ergebnisse. Die [Ressourcenrichtlinie](../resource-policy) begrenzt Ganzzahleingaben, Ausgabeskala und rationales Koeffizientenwachstum getrennt von Bibliotheksfehlern. Rust-Importsyntax ist ausdrücklich als nativ getestet gekennzeichnet.

## Verwandte Methoden

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
