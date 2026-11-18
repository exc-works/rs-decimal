---
locale: de
source_revision: 783c16e5cc4d9d8fc3c2d8c291b8f90a43746731baaf29ac25b9f64402f2b5c9
translation_provenance: en-US-german-reviewed
api_id: "Decimal::impl Neg"
---
# Decimal::impl Neg

Ausleihende Rust-Neg-Syntax verwenden.

```rust
impl Neg for & Decimal
```

## Übliche Verwendung und Hinweise

Für &Decimal implementiert, sodass Operanden nutzbar bleiben. Unäres - erhält die Skala und initialisiert das Ergebnis einschließlich Standardnull. Der Trait delegiert an dieselbe benannte Kernmethode. JavaScript-Operatoren im Browser sind nicht überladen; verwenden Sie add/sub/neg der Bridge.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-neg" locale="de" />

## Ausführung

Dieses Element ist native Rust-Trait-Syntax; es stellt keinen gleichnamigen JavaScript-Operator und kein SQLx-Objekt bereit. Das native Standard-Dokumentationstestprogramm führt dieses Beispiel aus.

## Verwandte Methoden

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
