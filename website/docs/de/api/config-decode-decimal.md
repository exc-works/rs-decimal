---
locale: de
source_revision: 0268b5969d6d0344c17f1c1c3dee925ce05b68c8ea0fc3782a3dde3b820e544c
translation_provenance: en-US-german-reviewed
api_id: "config::decode_decimal"
---
# config::decode_decimal

Explizit typisierte Konfigurationswerte decodieren.

```rust
fn decode_decimal (value : ConfigValue < '_ >) -> Result < Decimal , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

value: ConfigValue; liefert `Result<Decimal, DecimalError>`/ein neues Decimal-Handle. Rust benötigt config. Nicht-nullable-Decodierung weist Null und ungültige Nullable ab, statt den Zahlenwert null zu erfinden. Vorhandene Decimal und gültige Nullable werden mit vollständiger Darstellung einschließlich nicht initialisierter Null geklont. Text/Number/Bytes parsen exakten Dezimaltext ohne Entfernen von SQL-Anführungszeichen. Gleitkommawerte enthalten bereits binäre Rundung; verwenden Sie Text oder bigint für exakte Quellziffern. Browservarianten gehören zu einem Lauf und leihen Handles/Puffer bis zum Decodieren aus.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-decode-decimal" locale="de" />

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
