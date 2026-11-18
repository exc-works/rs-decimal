---
locale: de
source_revision: 7dc5d9b6c0f5030d430dc8fa4c821516f6b5bfe8979234b7de2b0cf0b1d59588
translation_provenance: en-US-german-reviewed
api_id: "Decimal::impl Type < Postgres >"
---
# Decimal::impl Type < Postgres >

Die native SQLx-PostgreSQL-NUMERIC-Anbindung verwenden.

```rust
impl Type < Postgres > for Decimal
```

## Übliche Verwendung und Hinweise

Benötigt das Feature postgres und die PostgreSQL-Typen von SQLx 0.8. Diese Implementierung ist rein nativ: Der Browser-Playground öffnet keine Datenbankverbindungen. type_info() liefert NUMERIC OID 1700. compatible(ty) akzeptiert genau diese OID, nicht float8, Ganzzahlen oder beliebige Texttypen; es wird keine implizite Gleitkommakonvertierung zugesagt.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-type-postgres" locale="de" />

## Ausführung

Dieses Element ist native Rust-Trait-Syntax; es stellt keinen gleichnamigen JavaScript-Operator und kein SQLx-Objekt bereit. Das native Standard-Dokumentationstestprogramm führt dieses Beispiel aus.

## Verwandte Methoden

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
