---
locale: de
source_revision: bc4fe0d7f63ebbaad6f009ea77ae93eee10d61514aa0fcf8907740ea4397dc70
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

Von Rust zurückgegebene Fehler werfen einen Error mit `name: 'DecimalError'`, `category: 'library'`, der ursprünglichen `message` und einem Zeichenfolgenarray `kinds`. Erhalten Sie das vollständige Array: Ein Decoderfehler kann mehreren Arten entsprechen. Fehler der JS-Typ- oder Bereichsprüfung haben dagegen `name: 'BridgeError'` und `category: 'parameter'`; sie sind keine Rust-Rechenfehler. Ungültige numerische Rundungsdiskriminanten erreichen Rusts Validierung.

Ausleihende Methoden lassen Objekte nutzbar. `into_rescaled` konsumiert seinen Empfänger auch dann, wenn Rust einen Fehler liefert. Nutzung eines konsumierten oder freigegebenen Werts löst einen BridgeError mit `moved` aus. Ungültige JS-Argumenttypen werden vor dem Aufruf abgewiesen und konsumieren den Empfänger nicht. `clone()` erhält Darstellung und Initialisierung. `free()` gibt aktive Handles frei und ist nach Konsumierung harmlos. Jeder Runner verwirft seine Wasm-Instanz bei Abschluss, ohne auf Finalizer zu vertrauen. Objekte einer anderen Bridge-Instanz können nicht übergeben werden.

## Nullable-Werte und Bytes

`NullDecimal.new(decimal)` konsumiert sein Argument und setzt `valid=true`, auch für nicht initialisierte Null. `NullDecimal.default()` bedeutet abwesend. Setzen von `valid` ändert nur die Gültigkeit und löscht den inneren Wert nicht. `get_decimal_clone()` ist ein explizit benannter Adapter für eine unabhängige Kopie. `set_decimal(value)` verschiebt einen neuen inneren Wert, ohne die Gültigkeit zu ändern. Es gibt keinen irreführenden live aktualisierten `.decimal`-Getter.

Auch `coefficient()` liefert einen unveränderlichen JS-bigint-Snapshot. JavaScript reproduziert weder Rust-Borrow-Lebensdauern noch Trait-Prüfungen zur Compilezeit. `ByteBuffer.new(Uint8Array)` kopiert Bytes in einen eigenen Puffer; `to_uint8_array()` liefert eine Kopie, die Wasm-Speicherwachstum übersteht. Append-Methoden stehen im [Methodenkatalog](./api/).

`snapshot(value)` auf Bridge-Ebene erfasst sofort den exakten Zustand, einschließlich verborgener Daten eines ungültigen nullable-Werts. Es liefert eingefrorene Daten; spätere Änderungen beeinflussen frühere Ausgaben nicht. Der isolierte Runner begrenzt rekursive Ausgaben und Ausführungszeit; siehe [Ressourcenrichtlinie](./resource-policy).
