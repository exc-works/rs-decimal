---
locale: de
source_revision: f56da33c81a26f95ee4ac8cfce759062606ade6eed7055366c2fe672d142ca01
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::impl Type < Postgres >::type_info"
---
# NullDecimal::impl Type < Postgres >::type_info

Die native SQLx-PostgreSQL-NUMERIC-Anbindung verwenden.

```rust
fn type_info () -> PgTypeInfo
```

## Übliche Verwendung und Hinweise

Benötigt das Feature postgres und die PostgreSQL-Typen von SQLx 0.8. Diese Implementierung ist rein nativ: Der Browser-Playground öffnet keine Datenbankverbindungen. type_info() liefert NUMERIC OID 1700. compatible(ty) akzeptiert genau diese OID, nicht float8, Ganzzahlen oder beliebige Texttypen; es wird keine implizite Gleitkommakonvertierung zugesagt.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-nulldecimal-impl-type-postgres-type-info" locale="de" />

## Ausführung

Dieses Element ist native Rust-Trait-Syntax; es stellt keinen gleichnamigen JavaScript-Operator und kein SQLx-Objekt bereit. Das native Standard-Dokumentationstestprogramm führt dieses Beispiel aus.

## Verwandte Methoden

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
