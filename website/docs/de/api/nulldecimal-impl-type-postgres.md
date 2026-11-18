---
locale: de
source_revision: 0830b6acb470b67cab11063b94daf8eafba63a697871936bf2d04b61331f067d
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::impl Type < Postgres >"
---
# NullDecimal::impl Type < Postgres >

Die native SQLx-PostgreSQL-NUMERIC-Anbindung verwenden.

```rust
impl Type < Postgres > for NullDecimal
```

## Übliche Verwendung und Hinweise

Benötigt das Feature postgres und die PostgreSQL-Typen von SQLx 0.8. Diese Implementierung ist rein nativ: Der Browser-Playground öffnet keine Datenbankverbindungen. type_info() liefert NUMERIC OID 1700. compatible(ty) akzeptiert genau diese OID, nicht float8, Ganzzahlen oder beliebige Texttypen; es wird keine implizite Gleitkommakonvertierung zugesagt.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-nulldecimal-impl-type-postgres" locale="de" />

## Ausführung

Dieses Element ist native Rust-Trait-Syntax; es stellt keinen gleichnamigen JavaScript-Operator und kein SQLx-Objekt bereit. Das native Standard-Dokumentationstestprogramm führt dieses Beispiel aus.

## Verwandte Methoden

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
