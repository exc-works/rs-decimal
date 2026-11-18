---
locale: de
source_revision: 7fadd186172ed1d2675ccc16ae4020af71fd55a578a479ca70e021285e76f370
translation_provenance: en-US-german-reviewed
api_id: "Decimal::impl From < BigInt >"
---
# Decimal::impl From < BigInt >

BigInt mit Rust From exakt konvertieren.

```rust
impl From < BigInt > for Decimal
```

## Übliche Verwendung und Hinweise

Konsumiert BigInt und liefert einen initialisierten Decimal mit Skala null. Es gibt weder Gleitkommazwischenschritt noch Rundung; die Konvertierung ist für den Quellganzzahltyp unfehlbar. Null ist initialisiert, anders als Decimal::default(). BigInt-Besitz geht in die Dezimalzahl über. Browsercode verwendet explizite benannte Konstruktoren oder signed/unsigned-Varianten von ConfigValue; Rust-From/Into-Syntax ist rein nativ.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-from-bigint" locale="de" />

## Ausführung

Dieses Element ist native Rust-Trait-Syntax; es stellt keinen gleichnamigen JavaScript-Operator und kein SQLx-Objekt bereit. Das native Standard-Dokumentationstestprogramm führt dieses Beispiel aus.

## Verwandte Methoden

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
