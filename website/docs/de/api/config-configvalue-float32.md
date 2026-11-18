---
locale: de
source_revision: d858bd2326ab43e0cc593d11fb421e19dddaa88d9dec2ce79a679a247e556178
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue::Float32"
---
# config::ConfigValue::Float32

Konfigurationseingabe Float32 wählen.

```rust
Float32 (f32)
```

## Parameter, Rückgabe und übliche Verwendung

binary32 transportieren. Rust verengt die Browser-number zuerst auf f32 und verwendet die Kernkonvertierung. Nicht endliche Eingabe oder Verengungsüberlauf liefert Unmarshal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-float32" locale="de" />

## Grenzen im Browser

Die Features config und serde sind aktiviert. Quelltext/-bytes sind auf 8192 UTF-8-Bytes begrenzt; Rust prüft Dezimaltext und decodierte JSON-Zeichenfolgen vor dem Parsen, sodass Escapes die [Ressourcenrichtlinie](../resource-policy) nicht umgehen. 128-Bit-Ganzzahlen benötigen bigint im gültigen Bereich. Ausgeliehene Decimal-/NullDecimal-Handles müssen beim Decodieren noch aktiv sein; die Konvertierung erzeugt unabhängige Werte ohne Eingabekonsum. Generisches Serde-Trait-Verhalten wird durch konkrete JSON-Adapter gezeigt.

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
