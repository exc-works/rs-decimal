---
locale: de
source_revision: 4de2b7f049c588d3fb290c5d299ab4e7f3436be10c0a26243bffab3f3e0229cf
translation_provenance: en-US-german-reviewed
api_id: "Decimal::impl Neg::Output"
---
# Decimal::impl Neg::Output

Ausleihende Rust-Neg-Syntax verwenden.

```rust
type Output = Decimal
```

## Übliche Verwendung und Hinweise

Für &Decimal implementiert, sodass Operanden nutzbar bleiben. Unäres - erhält die Skala und initialisiert das Ergebnis einschließlich Standardnull. Der Trait delegiert an dieselbe benannte Kernmethode. JavaScript-Operatoren im Browser sind nicht überladen; verwenden Sie add/sub/neg der JavaScript-API.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-neg-output" locale="de" />

## Ausführung

Dieses Element ist native Rust-Trait-Syntax; es stellt keinen gleichnamigen JavaScript-Operator und kein SQLx-Objekt bereit. Das native Standard-Dokumentationstestprogramm führt dieses Beispiel aus.

## Verwandte Methoden

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
