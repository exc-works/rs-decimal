---
locale: de
source_revision: 821c4c23615da2c03d94630c3843c11502e27fc2644faac01dc3a81234b52029
translation_provenance: en-US-german-reviewed
api_id: Decimal::float32
---
# Decimal::float32

Direkt nach IEEE binary32 konvertieren und Exaktheit melden.

```rust
fn float32 (& self) -> (f32 , bool)
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; Rust liefert (f32, bool), abgebildet auf [number, exact]. Gleichstände werden zur geraden Zahl gerundet. Das Kennzeichen bedeutet exakte Darstellung der Dezimalzahl, nicht bloßen Konvertierungserfolg: 0.1 ist unexakt. Überlauf liefert vorzeichenbehaftete Unendlichkeit mit false; Unterlauf kann vorzeichenbehaftete Null liefern. Die Rundung erfolgt direkt nach binary32, nicht zuerst nach binary64. Das Beispiel zeichnet IEEE-Bits auf, damit JSON Sonderwerte nicht verbirgt. Playground-print kennzeichnet auch NaN, Unendlichkeiten und negative Null mit ihren übertragenen binary64-Bits.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-float32" locale="de" />

## Grenzen im Browser

Ganzzahlen werden nicht über verlustbehaftete JS-Zahlen übertragen. Konvertierungstupel erhalten Rusts Reihenfolge Wert/Kennzeichen; false verspricht kein exaktes Ergebnis. Ausgeliehene Eingaben bleiben unverändert, Konstruktoren initialisieren Ergebnisse. Die [Ressourcenrichtlinie](../resource-policy) begrenzt Ganzzahleingaben, Ausgabeskala und rationales Koeffizientenwachstum getrennt von Bibliotheksfehlern. Rust-Importsyntax ist ausdrücklich als nativ getestet gekennzeichnet.

## Verwandte Methoden

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
