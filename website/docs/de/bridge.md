---
locale: de
source_revision: 967fc6e56bfb3ade48f0faf10b0c1e0de7e21a4d9ec093d537fb469385460f6f
translation_provenance: en-US-german-reviewed
---
# Rust-Methoden im Browser aufrufen

Der Playground führt JavaScript aus, das die nach Wasm kompilierte Rust-Bibliothek aufruft. Methodennamen, Parameterreihenfolge, Rundung und Dezimalzustand folgen Rust. JavaScript liefert den Kontrollfluss und berechnet keine Dezimalergebnisse. Einzelheiten stehen im [Methodenkatalog](./api/); Beispiele lassen sich im [Playground](./playground) ausführen.

```javascript
const price = Decimal.new_from_string('12.350');
const total = price.mul_exact(Decimal.new(3n));
total.string_with_trailing_zeros(); // '37.050'
```

Verwenden Sie exakte Zeichenfolgen für Dezimaleingaben und `bigint` für i64-Argumente (`3n`, nicht `3`). `Decimal.new` weist Werte außerhalb von i64, numerische Zeichenfolgen und JS-Zahlen ab. Präzision ist eine JS-Ganzzahl im vorzeichenbehafteten wasm32-Bereich. Gebrochene, nicht endliche und außerhalb des Bereichs liegende Eingaben werden abgewiesen, bevor ein ABI-Cast sie umbrechen oder abschneiden könnte. Negative ganzzahlige Präzision erreicht Rust und liefert dessen ursprünglichen Fehler InvalidPrecision.

## Fehler und Besitz

Das Array `kinds` kennzeichnet Rechenfehler wie `InvalidPrecision`. Hat ein Argument den falschen Typ oder liegt es außerhalb des gültigen Bereichs, lautet die Fehlerkategorie `parameter`.

Nach `rescale` ist der Ausgangswert weiter verwendbar. `into_rescaled` verbraucht ihn auch bei einem Rundungsfehler. Verwenden Sie vorher `clone`, um das Original zu behalten.

## Nullable-Werte und Bytes

`NullDecimal.new(decimal)` konsumiert sein Argument und setzt `valid=true`, auch für nicht initialisierte Null. `NullDecimal.default()` bedeutet abwesend. Setzen von `valid` ändert nur die Gültigkeit und löscht den inneren Wert nicht. `get_decimal_clone()` ist ein explizit benannter Adapter für eine unabhängige Kopie. `set_decimal(value)` verschiebt einen neuen inneren Wert, ohne die Gültigkeit zu ändern. Es gibt keinen irreführenden live aktualisierten `.decimal`-Getter.

Auch `coefficient()` liefert einen unveränderlichen JS-bigint-Snapshot. JavaScript reproduziert weder Rust-Borrow-Lebensdauern noch Trait-Prüfungen zur Compilezeit. `ByteBuffer.new(Uint8Array)` kopiert Bytes in einen eigenen Puffer; `to_uint8_array()` liefert eine Kopie, die Wasm-Speicherwachstum übersteht. Append-Methoden stehen im [Methodenkatalog](./api/).

`snapshot(value)`  erfasst sofort den exakten Zustand, einschließlich verborgener Daten eines ungültigen nullable-Werts. Es liefert eingefrorene Daten; spätere Änderungen beeinflussen frühere Ausgaben nicht. Der isolierte Runner begrenzt rekursive Ausgaben und Ausführungszeit; siehe [Ressourcenrichtlinie](./resource-policy).
