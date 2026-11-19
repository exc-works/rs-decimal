---
locale: de
source_revision: 3e32f687004e72286fa606992f7977ccbc210f3d36756c6006ab536cd5091f5a
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

## Hinweise zur Verwendung

Beim Kopieren von `Decimal::default()` bleibt der nicht initialisierte Zustand erhalten; es entsteht keine initialisierte Null. Übergeben Sie in JavaScript einen noch nutzbaren Decimal aus demselben Lauf. Dieser Konstruktor konsumiert ihn nicht.

## Verwandte Methoden

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
