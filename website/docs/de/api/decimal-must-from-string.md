---
locale: de
source_revision: 7805847022fc73cbb47e3960021e981cfb8ac4ab47a7b0a542ffa809d986fb9d
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

## Verwandte Methoden

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
