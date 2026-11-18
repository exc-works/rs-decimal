---
locale: de
source_revision: 584a930ae7d128f9ab20d43eb7ffff2b455374bd06ee0e33745542254e1599b0
translation_provenance: en-US-german-reviewed
api_id: "RoundingMode::impl TryFrom < i32 >::try_from"
---
# RoundingMode::impl TryFrom < i32 >::try_from

Einen rohen Rundungsdiskriminanten mit TryFrom validieren.

```rust
fn try_from (value : i32) -> Result < Self , Self :: Error >
```

## Übliche Verwendung und Hinweise

value: i32; liefert `Result<RoundingMode, DecimalError>`. Codes 0–6 verwenden die festgelegte Moduszuordnung einschließlich 5=HalfEven. Unbekannte Codes liefern InvalidArgument ohne Panic. Der zugehörige Error-Typ ist DecimalError. Delegiert an from_raw; Browsercode verwendet RoundingMode.from_raw(number) mit strenger i32-Parameterprüfung.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-roundingmode-impl-tryfrom-i32-try-from" locale="de" />

## Ausführung

Dieses Element ist native Rust-Trait-Syntax; es stellt keinen gleichnamigen JavaScript-Operator und kein SQLx-Objekt bereit. Das native Standard-Dokumentationstestprogramm führt dieses Beispiel aus.

## Verwandte Methoden

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
