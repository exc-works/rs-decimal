---
locale: de
source_revision: c4a0a7eefa609b0c57e86546936b34ea64a5ece51d5f57a28f19151facde5f69
translation_provenance: en-US-german-reviewed
api_id: "Decimal::impl Sub < & Decimal >::sub"
---
# Decimal::impl Sub < & Decimal >::sub

Ausleihende Rust-Sub-Syntax verwenden.

```rust
fn sub (self , other : & Decimal) -> Decimal
```

## Übliche Verwendung und Hinweise

Für &Decimal implementiert, sodass Operanden nutzbar bleiben. Das exakte Ergebnis verwendet die größere Operandenskala und ist initialisiert. Der zugehörige Output-Typ ist Decimal. Negative Ergebnisse sind erlaubt; die Nichtnegativitätsanforderungen von safe_add/safe_sub gelten hier nicht. Der Trait delegiert an dieselbe benannte Kernmethode. JavaScript-Operatoren im Browser sind nicht überladen; verwenden Sie add/sub/neg der JavaScript-API.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-sub-decimal-sub" locale="de" />

## Ausführung

Dieses Element ist native Rust-Trait-Syntax; es stellt keinen gleichnamigen JavaScript-Operator und kein SQLx-Objekt bereit. Das native Standard-Dokumentationstestprogramm führt dieses Beispiel aus.

## Verwandte Methoden

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
