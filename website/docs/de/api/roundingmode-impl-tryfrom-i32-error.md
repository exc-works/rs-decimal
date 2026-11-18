---
locale: de
source_revision: f397dc1220fc731959fe63aa3e5a1c46b0d68942bea1f3f5f567773d91995a1b
translation_provenance: en-US-german-reviewed
api_id: "RoundingMode::impl TryFrom < i32 >::Error"
---
# RoundingMode::impl TryFrom < i32 >::Error

Einen rohen Rundungsdiskriminanten mit TryFrom validieren.

```rust
type Error = DecimalError
```

## Übliche Verwendung und Hinweise

value: i32; liefert `Result<RoundingMode, DecimalError>`. Codes 0–6 verwenden die festgelegte Moduszuordnung einschließlich 5=HalfEven. Unbekannte Codes liefern InvalidArgument ohne Panic. Der zugehörige Error-Typ ist DecimalError. Delegiert an from_raw; Browsercode verwendet RoundingMode.from_raw(number) mit strenger i32-Parameterprüfung.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-roundingmode-impl-tryfrom-i32-error" locale="de" />

## Ausführung

Dieses Element ist native Rust-Trait-Syntax; es stellt keinen gleichnamigen JavaScript-Operator und kein SQLx-Objekt bereit. Das native Standard-Dokumentationstestprogramm führt dieses Beispiel aus.

## Verwandte Methoden

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
