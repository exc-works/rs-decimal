---
locale: de
source_revision: 437466c372c40ae5106ca331b0a6caec39f962da3ce28c0f9a2808a4f1f312aa
translation_provenance: en-US-german-reviewed
api_id: "Decimal::impl From < u128 >"
---
# Decimal::impl From < u128 >

u128 mit Rust From exakt konvertieren.

```rust
impl From<u128> for Decimal
```

## Übliche Verwendung und Hinweise

Konsumiert u128 und liefert einen initialisierten Decimal mit Skala null. Es gibt weder Gleitkommazwischenschritt noch Rundung; die Konvertierung ist für den Quellganzzahltyp unfehlbar. Null ist initialisiert, anders als Decimal::default(). Der Quellbereich entspricht dem Rust-Ganzzahltyp; die Breite von isize/usize hängt vom nativen Ziel ab. Browsercode verwendet explizite benannte Konstruktoren oder signed/unsigned-Varianten von ConfigValue; Rust-From/Into-Syntax ist rein nativ.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-from-u128" locale="de" />

## Ausführung

Dieses Element ist native Rust-Trait-Syntax; es stellt keinen gleichnamigen JavaScript-Operator und kein SQLx-Objekt bereit. Das native Standard-Dokumentationstestprogramm führt dieses Beispiel aus.

## Verwandte Methoden

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
