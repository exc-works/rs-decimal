---
locale: de
source_revision: 37b6ebebec69abdf2809e71eb7e3cad7353a792a069c7217103a51f2eea16c7f
translation_provenance: en-US-german-reviewed
api_id: Decimal::new_from_decimal
---
# Decimal::new_from_decimal

Einen vorhandenen Decimal einschließlich gespeicherter Darstellung und Initialisierung kopieren.

```rust
fn new_from_decimal (value : & Self) -> Self
```

## Parameter und Rückgabe

`value: &Decimal` wird ausgeliehen. Der zurückgegebene Decimal ist unabhängig; die Eingabe bleibt nutzbar.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-decimal" locale="de" />

## Hinweise und Grenzen

Beim Kopieren von `Decimal::default()` bleibt der nicht initialisierte Zustand erhalten; es entsteht keine initialisierte Null. Übergeben Sie in JavaScript einen noch nutzbaren Decimal aus demselben Lauf. Dieser Konstruktor konsumiert ihn nicht.

Der Browser wendet auch die [Ressourcenrichtlinie](../resource-policy) an. JavaScript-Eingaben werden vor dem Rust-Aufruf validiert.

## Verwandte Methoden

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
