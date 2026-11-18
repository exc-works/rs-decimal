---
locale: de
source_revision: b0ee5cc601aab9fac963006493ebe6b998c42307e354d93807217d424adc483c
translation_provenance: en-US-german-reviewed
api_id: "Decimal::impl From < i8 >"
---
# Decimal::impl From < i8 >

i8 mit Rust From exakt konvertieren.

```rust
impl From<i8> for Decimal
```

## Übliche Verwendung und Hinweise

Konsumiert i8 und liefert einen initialisierten Decimal mit Skala null. Es gibt weder Gleitkommazwischenschritt noch Rundung; die Konvertierung ist für den Quellganzzahltyp unfehlbar. Null ist initialisiert, anders als Decimal::default(). Der Quellbereich entspricht dem Rust-Ganzzahltyp; die Breite von isize/usize hängt vom nativen Ziel ab. Browsercode verwendet explizite benannte Konstruktoren oder signed/unsigned-Varianten von ConfigValue; Rust-From/Into-Syntax ist rein nativ.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-from-i8" locale="de" />

## Ausführung

Dieses Element ist native Rust-Trait-Syntax; es stellt keinen gleichnamigen JavaScript-Operator und kein SQLx-Objekt bereit. Das native Standard-Dokumentationstestprogramm führt dieses Beispiel aus.

## Verwandte Methoden

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
