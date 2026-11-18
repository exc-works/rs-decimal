---
locale: de
source_revision: 4bf7e8c6e0220b3e67f098890dd5d1be11f132b0691a01226108ed80b15e64dd
translation_provenance: en-US-german-reviewed
api_id: "Decimal::impl Sub < & Decimal >::Output"
---
# Decimal::impl Sub < & Decimal >::Output

Ausleihende Rust-Sub-Syntax verwenden.

```rust
type Output = Decimal
```

## Übliche Verwendung und Hinweise

Für &Decimal implementiert, sodass Operanden nutzbar bleiben. Das exakte Ergebnis verwendet die größere Operandenskala und ist initialisiert. Der zugehörige Output-Typ ist Decimal. Negative Ergebnisse sind erlaubt; die Nichtnegativitätsanforderungen von safe_add/safe_sub gelten hier nicht. Der Trait delegiert an dieselbe benannte Kernmethode. JavaScript-Operatoren im Browser sind nicht überladen; verwenden Sie add/sub/neg der Bridge.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-sub-decimal-output" locale="de" />

## Ausführung

Dieses Element ist native Rust-Trait-Syntax; es stellt keinen gleichnamigen JavaScript-Operator und kein SQLx-Objekt bereit. Das native Standard-Dokumentationstestprogramm führt dieses Beispiel aus.

## Verwandte Methoden

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
