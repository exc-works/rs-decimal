---
locale: de
source_revision: ae30eee52b8243c53d2363e82db8a8847f306fed0233c1fadf3d83f102fe5452
translation_provenance: en-US-german-reviewed
api_id: "Decimal::impl Neg::neg"
---
# Decimal::impl Neg::neg

Ausleihende Rust-Neg-Syntax verwenden.

```rust
fn neg (self) -> Decimal
```

## Übliche Verwendung und Hinweise

Für &Decimal implementiert, sodass Operanden nutzbar bleiben. Unäres - erhält die Skala und initialisiert das Ergebnis einschließlich Standardnull. Der Trait delegiert an dieselbe benannte Kernmethode. JavaScript-Operatoren im Browser sind nicht überladen; verwenden Sie add/sub/neg der JavaScript-API.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-neg-neg" locale="de" />

## Ausführung

Dieses Element ist native Rust-Trait-Syntax; es stellt keinen gleichnamigen JavaScript-Operator und kein SQLx-Objekt bereit. Das native Standard-Dokumentationstestprogramm führt dieses Beispiel aus.

## Verwandte Methoden

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
