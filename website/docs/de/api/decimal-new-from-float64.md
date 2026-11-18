---
locale: de
source_revision: 52c061e67f4d78f9abff6cb38b422cdd828bc88de770dccac8c7ef157f4fde94
translation_provenance: en-US-german-reviewed
api_id: Decimal::new_from_float64
---
# Decimal::new_from_float64

Aus der kürzesten Dezimaldarstellung konstruieren, die wieder denselben Gleitkommawert ergibt.

```rust
fn new_from_float64 (value : f64) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

value: f64, als JS number übergeben. Bereits durch binäre Gleitkommadarstellung verlorene Dezimalziffern werden nicht wiederhergestellt. Verwenden Sie new_from_string für exakte Dezimaleingaben. NaN und beide Unendlichkeiten liefern InvalidFormat. Negative Null wird zur initialisierten Decimal-Null ohne Vorzeichen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-float64" locale="de" />

## Grenzen im Browser

Ganzzahlen werden nicht über verlustbehaftete JS-Zahlen übertragen. Konvertierungstupel erhalten Rusts Reihenfolge Wert/Kennzeichen; false verspricht kein exaktes Ergebnis. Ausgeliehene Eingaben bleiben unverändert, Konstruktoren initialisieren Ergebnisse. Die [Ressourcenrichtlinie](../resource-policy) begrenzt Ganzzahleingaben, Ausgabeskala und rationales Koeffizientenwachstum getrennt von Bibliotheksfehlern. Rust-Importsyntax ist ausdrücklich als nativ getestet gekennzeichnet.

## Verwandte Methoden

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
