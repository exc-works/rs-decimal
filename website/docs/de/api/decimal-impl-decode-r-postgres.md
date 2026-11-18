---
locale: de
source_revision: 8a46a10605089198b3fd10258613908601241fb4fcf1464a3ecf390fc7fa9450
translation_provenance: en-US-german-reviewed
api_id: "Decimal::impl Decode < 'r , Postgres >"
---
# Decimal::impl Decode < 'r , Postgres >

Die native SQLx-PostgreSQL-NUMERIC-Anbindung verwenden.

```rust
impl< 'r > Decode < 'r , Postgres > for Decimal
```

## Übliche Verwendung und Hinweise

Benötigt das Feature postgres und die PostgreSQL-Typen von SQLx 0.8. Diese Implementierung ist rein nativ: Der Browser-Playground öffnet keine Datenbankverbindungen. decode(PgValueRef) liefert `Result<Self, BoxDynError>`. SQLx stellt den ausgeliehenen Datenbankwert bereit; erzeugen Sie dessen private Übertragungsdarstellung nicht selbst. Binäres und textuelles NUMERIC erhalten Wert und Anzeigeskala einschließlich skalierter Null. SQL NULL wird abgewiesen; verwenden Sie NullDecimal oder `Option<Decimal>` für nullable-Spalten. Ergebnisse außer null sind initialisiert. Fehlerhafte oder nicht endliche NUMERIC-Werte werden ohne Rundung abgewiesen. Dieses Beispiel läuft in CI gegen einen dedizierten PostgreSQL-Server; der Browser benötigt keine externe Datenbank.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-decode-r-postgres" locale="de" />

## Ausführung

Dieses Element ist native Rust-Trait-Syntax; es stellt keinen gleichnamigen JavaScript-Operator und kein SQLx-Objekt bereit. Führen Sie das Beispiel mit einem dedizierten PostgreSQL-Server, DATABASE_URL und `cargo run --locked -p decimal-web-examples --features postgres-live` aus. Ohne dieses Profil wird es ausdrücklich als übersprungen gemeldet und nicht als bestandene Datenbankprüfung gezählt.

## Verwandte Methoden

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
