---
locale: de
source_revision: e38006128b404f87e5b765e2fa64c4d3f058b23c6a114c6e9a68f4ed1634f91f
translation_provenance: en-US-german-reviewed
api_id: "RoundingMode::impl TryFrom < i32 >"
---
# RoundingMode::impl TryFrom < i32 >

Einen rohen Rundungsdiskriminanten mit TryFrom validieren.

```rust
impl TryFrom < i32 > for RoundingMode
```

## Übliche Verwendung und Hinweise

value: i32; liefert `Result<RoundingMode, DecimalError>`. Codes 0–6 verwenden die festgelegte Moduszuordnung einschließlich 5=HalfEven. Unbekannte Codes liefern InvalidArgument ohne Panic. Der zugehörige Error-Typ ist DecimalError. Delegiert an from_raw; Browsercode verwendet RoundingMode.from_raw(number) mit strenger i32-Parameterprüfung.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-roundingmode-impl-tryfrom-i32" locale="de" />

## Ausführung

Dieses Element ist native Rust-Trait-Syntax; es stellt keinen gleichnamigen JavaScript-Operator und kein SQLx-Objekt bereit. Das native Standard-Dokumentationstestprogramm führt dieses Beispiel aus.

## Verwandte Methoden

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
