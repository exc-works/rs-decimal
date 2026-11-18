---
locale: de
source_revision: 17e780c5fcd60ccd7701083f059fdd79ff3607aa9dff1b3dfa127b396f366781
translation_provenance: en-US-german-reviewed
api_id: Decimal::new_from_float32
---
# Decimal::new_from_float32

Aus der kürzesten Dezimaldarstellung konstruieren, die wieder denselben Gleitkommawert ergibt.

```rust
fn new_from_float32 (value : f32) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

value: f32, als JS number übergeben. Die Bridge verengt zunächst binary64 auf binary32; 16777217 wird dadurch 16777216. Bereits durch binäre Gleitkommadarstellung verlorene Dezimalziffern werden nicht wiederhergestellt. Verwenden Sie new_from_string für exakte Dezimaleingaben. NaN und beide Unendlichkeiten liefern InvalidFormat. Ein endliches f64, das bei Verengung auf f32 überläuft, liefert ebenfalls InvalidFormat. Negative Null wird zur initialisierten Decimal-Null ohne Vorzeichen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-float32" locale="de" />

## Grenzen im Browser

Ganzzahlen werden nicht über verlustbehaftete JS-Zahlen übertragen. Konvertierungstupel erhalten Rusts Reihenfolge Wert/Kennzeichen; false verspricht kein exaktes Ergebnis. Ausgeliehene Eingaben bleiben unverändert, Konstruktoren initialisieren Ergebnisse. Die [Ressourcenrichtlinie](../resource-policy) begrenzt Ganzzahleingaben, Ausgabeskala und rationales Koeffizientenwachstum getrennt von Bibliotheksfehlern. Rust-Importsyntax ist ausdrücklich als nativ getestet gekennzeichnet.

## Verwandte Methoden

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
