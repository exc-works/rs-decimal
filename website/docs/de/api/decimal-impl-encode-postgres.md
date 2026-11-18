---
locale: de
source_revision: ca5166a5653606ecffcf807579adb23570a7902ad8051649236d87831df9c02a
translation_provenance: en-US-german-reviewed
api_id: "Decimal::impl Encode < '_ , Postgres >"
---
# Decimal::impl Encode < '_ , Postgres >

Die native SQLx-PostgreSQL-NUMERIC-Anbindung verwenden.

```rust
impl Encode < '_ , Postgres > for Decimal
```

## Übliche Verwendung und Hinweise

Benötigt das Feature postgres und die PostgreSQL-Typen von SQLx 0.8. Diese Implementierung ist rein nativ: Der Browser-Playground öffnet keine Datenbankverbindungen. encode_by_ref(&self, &mut PgArgumentBuffer) liefert `Result<IsNull, BoxDynError>`. Ausleihen erhält den Wert; Bytes werden erst nach erfolgreichem Codieren an den vorhandenen Argumentpuffer angehängt. Auch nicht initialisierter Decimal wird als nicht-NULL NUMERIC-Null codiert. Das Binärformat verwendet Big-Endian-Gruppen zur Basis 10000, erhält die Anzeigeskala und verwendet keinen float-/BigDecimal-Zwischenschritt. Skala über 16383 oder mehr als 131072 Ganzzahlziffern werden abgewiesen; Codierfehler erhalten den Zielpuffer.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-encode-postgres" locale="de" />

## Ausführung

Dieses Element ist native Rust-Trait-Syntax; es stellt keinen gleichnamigen JavaScript-Operator und kein SQLx-Objekt bereit. Das native Standard-Dokumentationstestprogramm führt dieses Beispiel aus.

## Verwandte Methoden

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
