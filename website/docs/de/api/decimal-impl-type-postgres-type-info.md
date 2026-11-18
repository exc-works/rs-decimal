---
locale: de
source_revision: 841ef20b63498074bad543fcbeb5b16a645ffde5b38df63feb8a348e6536be41
translation_provenance: en-US-german-reviewed
api_id: "Decimal::impl Type < Postgres >::type_info"
---
# Decimal::impl Type < Postgres >::type_info

Die native SQLx-PostgreSQL-NUMERIC-Anbindung verwenden.

```rust
fn type_info () -> PgTypeInfo
```

## Übliche Verwendung und Hinweise

Benötigt das Feature postgres und die PostgreSQL-Typen von SQLx 0.8. Diese Implementierung ist rein nativ: Der Browser-Playground öffnet keine Datenbankverbindungen. type_info() liefert NUMERIC OID 1700. compatible(ty) akzeptiert genau diese OID, nicht float8, Ganzzahlen oder beliebige Texttypen; es wird keine implizite Gleitkommakonvertierung zugesagt.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-type-postgres-type-info" locale="de" />

## Ausführung

Dieses Element ist native Rust-Trait-Syntax; es stellt keinen gleichnamigen JavaScript-Operator und kein SQLx-Objekt bereit. Das native Standard-Dokumentationstestprogramm führt dieses Beispiel aus.

## Verwandte Methoden

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
