---
locale: de
source_revision: 1e0fd629af73a5e538f91b272bef72197cbcd10de91d5bebf50a93024747c0fa
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::impl PgHasArrayType::array_type_info"
---
# NullDecimal::impl PgHasArrayType::array_type_info

Die native SQLx-PostgreSQL-NUMERIC-Anbindung verwenden.

```rust
fn array_type_info () -> PgTypeInfo
```

## Übliche Verwendung und Hinweise

Benötigt das Feature postgres und die PostgreSQL-Typen von SQLx 0.8. Diese Implementierung ist rein nativ: Der Browser-Playground öffnet keine Datenbankverbindungen. array_type_info() liefert NUMERIC[] OID 1231 und ermöglicht SQLx-Arraybindungen. NullDecimal-Arrayelemente erhalten SQL NULL über die Gültigkeit; verwenden Sie native SQLx-Abfragen zum Codieren/Decodieren vollständiger Arrays.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-nulldecimal-impl-pghasarraytype-array-type-info" locale="de" />

## Ausführung

Dieses Element ist native Rust-Trait-Syntax; es stellt keinen gleichnamigen JavaScript-Operator und kein SQLx-Objekt bereit. Das native Standard-Dokumentationstestprogramm führt dieses Beispiel aus.

## Verwandte Methoden

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
