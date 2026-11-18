---
locale: de
source_revision: 78ca92acc2a8523b4e668da7c55a57a56b16410b6455b88e11d916edee01152c
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue::Null"
---
# config::ConfigValue::Null

Konfigurationseingabe Null wählen.

```rust
Null
```

## Parameter, Rückgabe und übliche Verwendung

Eine fehlende Konfigurationseingabe. decode_nullable liefert einen ungültigen Wert; decode_decimal weist sie mit Unmarshal ab.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-null" locale="de" />

## Grenzen im Browser

Die Features config und serde sind aktiviert. Quelltext/-bytes sind auf 8192 UTF-8-Bytes begrenzt; Rust prüft Dezimaltext und decodierte JSON-Zeichenfolgen vor dem Parsen, sodass Escapes die [Ressourcenrichtlinie](../resource-policy) nicht umgehen. 128-Bit-Ganzzahlen benötigen bigint im gültigen Bereich. Ausgeliehene Decimal-/NullDecimal-Handles müssen beim Decodieren noch aktiv sein; die Konvertierung erzeugt unabhängige Werte ohne Eingabekonsum. Generisches Serde-Trait-Verhalten wird durch konkrete JSON-Adapter gezeigt.

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
