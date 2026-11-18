---
locale: de
source_revision: f24ab15a12ac3cc5edffa3bb6354648be7a4944a94929ac82037f54fe89c586d
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::impl Encode < '_ , Postgres >::encode_by_ref"
---
# NullDecimal::impl Encode < '_ , Postgres >::encode_by_ref

Die native SQLx-PostgreSQL-NUMERIC-Anbindung verwenden.

```rust
fn encode_by_ref (& self , buf : & mut PgArgumentBuffer) -> Result < IsNull , BoxDynError >
```

## Übliche Verwendung und Hinweise

Benötigt das Feature postgres und die PostgreSQL-Typen von SQLx 0.8. Diese Implementierung ist rein nativ: Der Browser-Playground öffnet keine Datenbankverbindungen. encode_by_ref(&self, &mut PgArgumentBuffer) liefert `Result<IsNull, BoxDynError>`. Ausleihen erhält den Wert; Bytes werden erst nach erfolgreichem Codieren an den vorhandenen Argumentpuffer angehängt. Ungültige nullable-Werte liefern IsNull::Yes ohne Bytes. Gültige Werte codieren den inneren Decimal, auch bei nicht initialisierter Null. Das Binärformat verwendet Big-Endian-Gruppen zur Basis 10000, erhält die Anzeigeskala und verwendet keinen float-/BigDecimal-Zwischenschritt. Skala über 16383 oder mehr als 131072 Ganzzahlziffern werden abgewiesen; Codierfehler erhalten den Zielpuffer.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-nulldecimal-impl-encode-postgres-encode-by-ref" locale="de" />

## Ausführung

Dieses Element ist native Rust-Trait-Syntax; es stellt keinen gleichnamigen JavaScript-Operator und kein SQLx-Objekt bereit. Das native Standard-Dokumentationstestprogramm führt dieses Beispiel aus.

## Verwandte Methoden

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
