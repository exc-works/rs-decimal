---
locale: de
source_revision: 3ef0ab66b1050d81978ffe9504c298d164d62d052b64bfd1ad02ddce4a039f36
translation_provenance: en-US-german-reviewed
api_id: Decimal::new_from_string
---
# Decimal::new_from_string

Exakten Dezimaltext ohne binären Gleitkommaumweg einlesen. Dies ist der übliche Konstruktor für vom Benutzer eingegebene Dezimalbeträge.

```rust
fn new_from_string (text : & str) -> Result < Self , DecimalError >
```

## Parameter und Rückgabe

`text: &str` → JavaScript `string`; Rust liefert `Result<Decimal, DecimalError>`, die Bridge einen Decimal oder eine Ausnahme mit denselben Bibliothekskategorien.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-string" locale="de" />

## Hinweise und Grenzen

Äußerer Leerraum wird entfernt; Vorzeichen und Exponent sind erlaubt. Auf beiden Seiten des Dezimalpunkts sind Ziffern nötig. Das Trennzeichen ist unabhängig von der Oberflächensprache stets `.`. Bei Werten ungleich null bleiben nachgestellte Nullen erhalten, aber `"0.00"` wird zur initialisierten Null mit Skala null normalisiert. Ungültige Syntax liefert `InvalidFormat`; expandierte Eingaben können die separate Ressourcengrenze des Playgrounds erreichen.

Der Browser wendet auch die [Ressourcenrichtlinie](../resource-policy) an. JavaScript-Eingaben werden vor dem Rust-Aufruf validiert.

## Verwandte Methoden

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
