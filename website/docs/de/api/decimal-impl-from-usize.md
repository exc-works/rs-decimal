---
locale: de
source_revision: 910a84a6ded31baaefbfbdf3f7b025090c5c19a248c0ddcdb4d560e598525806
translation_provenance: en-US-german-reviewed
api_id: "Decimal::impl From < usize >"
---
# Decimal::impl From < usize >

usize mit Rust From exakt konvertieren.

```rust
impl From<usize> for Decimal
```

## Übliche Verwendung und Hinweise

Konsumiert usize und liefert einen initialisierten Decimal mit Skala null. Es gibt weder Gleitkommazwischenschritt noch Rundung; die Konvertierung ist für den Quellganzzahltyp unfehlbar. Null ist initialisiert, anders als Decimal::default(). Der Quellbereich entspricht dem Rust-Ganzzahltyp; die Breite von isize/usize hängt vom nativen Ziel ab. Browsercode verwendet explizite benannte Konstruktoren oder signed/unsigned-Varianten von ConfigValue; Rust-From/Into-Syntax ist rein nativ.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-from-usize" locale="de" />

## Ausführung

Dieses Element ist native Rust-Trait-Syntax; es stellt keinen gleichnamigen JavaScript-Operator und kein SQLx-Objekt bereit. Das native Standard-Dokumentationstestprogramm führt dieses Beispiel aus.

## Verwandte Methoden

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
