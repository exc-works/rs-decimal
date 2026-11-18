---
locale: de
source_revision: b6da66eb1330fd7e72548ba45454893fec7b7bdb425b5a72d19b09887a6620cd
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue::Unsigned"
---
# config::ConfigValue::Unsigned

Konfigurationseingabe Unsigned wählen.

```rust
Unsigned (u128)
```

## Parameter, Rückgabe und übliche Verwendung

Ein vorzeichenloses u128 exakt transportieren. Der Browser verlangt bigint in [0, 2^128-1], ohne Gleitkommazwischenschritt.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-unsigned" locale="de" />

## Grenzen im Browser

Die Features config und serde sind aktiviert. Quelltext/-bytes sind auf 8192 UTF-8-Bytes begrenzt; Rust prüft Dezimaltext und decodierte JSON-Zeichenfolgen vor dem Parsen, sodass Escapes die [Ressourcenrichtlinie](../resource-policy) nicht umgehen. 128-Bit-Ganzzahlen benötigen bigint im gültigen Bereich. Ausgeliehene Decimal-/NullDecimal-Handles müssen beim Decodieren noch aktiv sein; die Konvertierung erzeugt unabhängige Werte ohne Eingabekonsum. Generisches Serde-Trait-Verhalten wird durch konkrete JSON-Adapter gezeigt.

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
