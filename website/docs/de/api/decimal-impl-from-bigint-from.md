---
locale: de
source_revision: 97e436f7da1b6009e290487900a95055ccb9df22c09952a8d25d64cf66caf012
translation_provenance: en-US-german-reviewed
api_id: "Decimal::impl From < BigInt >::from"
---
# Decimal::impl From < BigInt >::from

BigInt mit Rust From exakt konvertieren.

```rust
fn from (value : BigInt) -> Self
```

## Übliche Verwendung und Hinweise

Konsumiert BigInt und liefert einen initialisierten Decimal mit Skala null. Es gibt weder Gleitkommazwischenschritt noch Rundung; die Konvertierung ist für den Quellganzzahltyp unfehlbar. Null ist initialisiert, anders als Decimal::default(). BigInt-Besitz geht in die Dezimalzahl über. Browsercode verwendet explizite benannte Konstruktoren oder signed/unsigned-Varianten von ConfigValue; Rust-From/Into-Syntax ist rein nativ.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-from-bigint-from" locale="de" />

## Ausführung

Dieses Element ist native Rust-Trait-Syntax; es stellt keinen gleichnamigen JavaScript-Operator und kein SQLx-Objekt bereit. Das native Standard-Dokumentationstestprogramm führt dieses Beispiel aus.

## Verwandte Methoden

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
