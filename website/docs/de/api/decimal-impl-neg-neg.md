---
locale: de
source_revision: 4edb7e0a9fed4047c94ce74a3f224028a33e2b9c87f41b5e744d6d68a1abd2e7
translation_provenance: en-US-german-reviewed
api_id: "Decimal::impl Neg::neg"
---
# Decimal::impl Neg::neg

Ausleihende Rust-Neg-Syntax verwenden.

```rust
fn neg (self) -> Decimal
```

## Übliche Verwendung und Hinweise

Für &Decimal implementiert, sodass Operanden nutzbar bleiben. Unäres - erhält die Skala und initialisiert das Ergebnis einschließlich Standardnull. Der Trait delegiert an dieselbe benannte Kernmethode. JavaScript-Operatoren im Browser sind nicht überladen; verwenden Sie add/sub/neg der Bridge.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-neg-neg" locale="de" />

## Ausführung

Dieses Element ist native Rust-Trait-Syntax; es stellt keinen gleichnamigen JavaScript-Operator und kein SQLx-Objekt bereit. Das native Standard-Dokumentationstestprogramm führt dieses Beispiel aus.

## Verwandte Methoden

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
