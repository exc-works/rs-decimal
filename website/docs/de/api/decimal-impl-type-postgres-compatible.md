---
locale: de
source_revision: 08d21be3db40ef9491657e8f165b85a45fc054adfc77bf6c387cc33e2075f02e
translation_provenance: en-US-german-reviewed
api_id: "Decimal::impl Type < Postgres >::compatible"
---
# Decimal::impl Type < Postgres >::compatible

Die native SQLx-PostgreSQL-NUMERIC-Anbindung verwenden.

```rust
fn compatible (ty : & PgTypeInfo) -> bool
```

## Übliche Verwendung und Hinweise

Benötigt das Feature postgres und die PostgreSQL-Typen von SQLx 0.8. Diese Implementierung ist rein nativ: Der Browser-Playground öffnet keine Datenbankverbindungen. type_info() liefert NUMERIC OID 1700. compatible(ty) akzeptiert genau diese OID, nicht float8, Ganzzahlen oder beliebige Texttypen; es wird keine implizite Gleitkommakonvertierung zugesagt.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-type-postgres-compatible" locale="de" />

## Ausführung

Dieses Element ist native Rust-Trait-Syntax; es stellt keinen gleichnamigen JavaScript-Operator und kein SQLx-Objekt bereit. Das native Standard-Dokumentationstestprogramm führt dieses Beispiel aus.

## Verwandte Methoden

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
