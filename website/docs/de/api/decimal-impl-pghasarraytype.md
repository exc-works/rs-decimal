---
locale: de
source_revision: 02d9f7c1451db386b5c6959aa8f2c1d403c00069d00df185eafd17eb7f7b188e
translation_provenance: en-US-german-reviewed
api_id: "Decimal::impl PgHasArrayType"
---
# Decimal::impl PgHasArrayType

Die native SQLx-PostgreSQL-NUMERIC-Anbindung verwenden.

```rust
impl PgHasArrayType for Decimal
```

## Übliche Verwendung und Hinweise

Benötigt das Feature postgres und die PostgreSQL-Typen von SQLx 0.8. Diese Implementierung ist rein nativ: Der Browser-Playground öffnet keine Datenbankverbindungen. array_type_info() liefert NUMERIC[] OID 1231 und ermöglicht SQLx-Arraybindungen. NullDecimal-Arrayelemente erhalten SQL NULL über die Gültigkeit; verwenden Sie native SQLx-Abfragen zum Codieren/Decodieren vollständiger Arrays.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-pghasarraytype" locale="de" />

## Ausführung

Dieses Element ist native Rust-Trait-Syntax; es stellt keinen gleichnamigen JavaScript-Operator und kein SQLx-Objekt bereit. Das native Standard-Dokumentationstestprogramm führt dieses Beispiel aus.

## Verwandte Methoden

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
