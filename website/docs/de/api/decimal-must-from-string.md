---
locale: de
source_revision: bb2070c90fbcf81b9f15485debb923fef5e838831e150577f2d283f059db73b6
translation_provenance: en-US-german-reviewed
api_id: Decimal::must_from_string
---
# Decimal::must_from_string

Vertrauenswürdigen Dezimaltext einlesen, wenn ungültige Eingaben Programmierfehler sind.

```rust
fn must_from_string (text : & str) -> Self
```

## Parameter, Rückgabe und Hinweise

text: &str/string; liefert Decimal. Gültige Eingaben haben denselben Zustand wie bei new_from_string. Ungültige Syntax löst in Rust einen Panic mit DecimalError aus; kompiliertes Wasm erzeugt einen Trap statt eines behandelbaren Bibliotheksfehlers. Verwenden Sie für Benutzereingaben bevorzugt new_from_string. Ein unbehandelter Trap beendet den aktuellen Lauf; starten Sie vor erneuter Rust-Nutzung einen frischen Lauf. Parameter- und Ressourcenprüfungen weisen Eingaben weiterhin vor Rust ab.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-decimal-must-from-string" locale="de" />

## Grenzen

Die Bridge erhält Rust-Namen und Parameterreihenfolge. Freie Funktionen stehen im Playground direkt bereit. Ausgeliehene Eingaben werden weder konsumiert noch verändert; ungültige Handles oder solche aus fremden Läufen werden abgewiesen. Alle Operationen folgen der [Ressourcenrichtlinie](../resource-policy).

## Verwandte Methoden

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
