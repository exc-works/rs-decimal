---
locale: de
source_revision: 03da42d48c5d8b250daaaa93d59162466d4ca8b88da11e32c7b577b2d36d0747
translation_provenance: en-US-german-reviewed
api_id: "config"
---
# config

Explizit typisierte Konfigurationswerte decodieren.

```rust
pub mod config
```

## Parameter, Rückgabe und übliche Verwendung

Der typisierte Konfigurationsadapter unterstützt Text/Textbytes, exakten numerischen Text, vorzeichenbehaftete/vorzeichenlose 128-Bit-Ganzzahlen, Gleitkommawerte und ausgeliehene Decimal-/NullDecimal-Werte. Rust benötigt config. Nicht-nullable-Decodierung weist Null und ungültige Nullable ab, statt den Zahlenwert null zu erfinden. Vorhandene Decimal und gültige Nullable werden mit vollständiger Darstellung einschließlich nicht initialisierter Null geklont. Text/Number/Bytes parsen exakten Dezimaltext ohne Entfernen von SQL-Anführungszeichen. Gleitkommawerte enthalten bereits binäre Rundung; verwenden Sie Text oder bigint für exakte Quellziffern. Browservarianten gehören zu einem Lauf und leihen Handles/Puffer bis zum Decodieren aus.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config" locale="de" />

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
