---
locale: de
source_revision: fe8961def1f2d55b79394ddbdd152dccfbb21eaad02a1ae36eaa6898f24726be
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue"
---
# config::ConfigValue

Explizit typisierte Konfigurationswerte decodieren.

```rust
pub enum ConfigValue< 'a >
```

## Parameter, Rückgabe und übliche Verwendung

Der typisierte Konfigurationsadapter unterstützt Text/Textbytes, exakten numerischen Text, vorzeichenbehaftete/vorzeichenlose 128-Bit-Ganzzahlen, Gleitkommawerte und ausgeliehene Decimal-/NullDecimal-Werte. Rust benötigt config. Nicht-nullable-Decodierung weist Null und ungültige Nullable ab, statt den Zahlenwert null zu erfinden. Vorhandene Decimal und gültige Nullable werden mit vollständiger Darstellung einschließlich nicht initialisierter Null geklont. Text/Number/Bytes parsen exakten Dezimaltext ohne Entfernen von SQL-Anführungszeichen. Gleitkommawerte enthalten bereits binäre Rundung; verwenden Sie Text oder bigint für exakte Quellziffern. Browservarianten gehören zu einem Lauf und leihen Handles/Puffer bis zum Decodieren aus.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue" locale="de" />

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
