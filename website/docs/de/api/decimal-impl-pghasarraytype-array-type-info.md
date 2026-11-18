---
locale: de
source_revision: 385e2c824095efc3d2ce40587d64168818dba7b455d7f53f60a47309b6e48343
translation_provenance: en-US-german-reviewed
api_id: "Decimal::impl PgHasArrayType::array_type_info"
---
# Decimal::impl PgHasArrayType::array_type_info

Die native SQLx-PostgreSQL-NUMERIC-Anbindung verwenden.

```rust
fn array_type_info () -> PgTypeInfo
```

## Übliche Verwendung und Hinweise

Benötigt das Feature postgres und die PostgreSQL-Typen von SQLx 0.8. Diese Implementierung ist rein nativ: Der Browser-Playground öffnet keine Datenbankverbindungen. array_type_info() liefert NUMERIC[] OID 1231 und ermöglicht SQLx-Arraybindungen. NullDecimal-Arrayelemente erhalten SQL NULL über die Gültigkeit; verwenden Sie native SQLx-Abfragen zum Codieren/Decodieren vollständiger Arrays.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-pghasarraytype-array-type-info" locale="de" />

## Ausführung

Dieses Element ist native Rust-Trait-Syntax; es stellt keinen gleichnamigen JavaScript-Operator und kein SQLx-Objekt bereit. Das native Standard-Dokumentationstestprogramm führt dieses Beispiel aus.

## Verwandte Methoden

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
