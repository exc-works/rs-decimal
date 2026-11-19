---
locale: de
source_revision: 25c9cc1e70c5db48b15f2e6457931c51bc64818138c0422f347742831125e26b
translation_provenance: en-US-german-reviewed
api_id: "Decimal::impl Add < & Decimal >::add"
---
# Decimal::impl Add < & Decimal >::add

Ausleihende Rust-Add-Syntax verwenden.

```rust
fn add (self , other : & Decimal) -> Decimal
```

## Übliche Verwendung und Hinweise

Für &Decimal implementiert, sodass Operanden nutzbar bleiben. Das exakte Ergebnis verwendet die größere Operandenskala und ist initialisiert. Der zugehörige Output-Typ ist Decimal. Negative Ergebnisse sind erlaubt; die Nichtnegativitätsanforderungen von safe_add/safe_sub gelten hier nicht. Der Trait delegiert an dieselbe benannte Kernmethode. JavaScript-Operatoren im Browser sind nicht überladen; verwenden Sie add/sub/neg der JavaScript-API.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-add-decimal-add" locale="de" />

## Ausführung

Dieses Element ist native Rust-Trait-Syntax; es stellt keinen gleichnamigen JavaScript-Operator und kein SQLx-Objekt bereit. Das native Standard-Dokumentationstestprogramm führt dieses Beispiel aus.

## Verwandte Methoden

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
