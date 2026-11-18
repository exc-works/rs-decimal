---
locale: de
source_revision: 21a2321ebb07dad0e89ac689f0921883a544f6575bbd19e235e51c9aee9deb73
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue::Float64"
---
# config::ConfigValue::Float64

Konfigurationseingabe Float64 wählen.

```rust
Float64 (f64)
```

## Parameter, Rückgabe und übliche Verwendung

binary64 über die kürzeste Dezimalkonvertierung des Kerns transportieren. Nicht endliche Werte liefern Unmarshal; durch Gleitkommarundung bereits verlorene Ziffern sind nicht wiederherstellbar.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-float64" locale="de" />

## Grenzen im Browser

Die Features config und serde sind aktiviert. Quelltext/-bytes sind auf 8192 UTF-8-Bytes begrenzt; Rust prüft Dezimaltext und decodierte JSON-Zeichenfolgen vor dem Parsen, sodass Escapes die [Ressourcenrichtlinie](../resource-policy) nicht umgehen. 128-Bit-Ganzzahlen benötigen bigint im gültigen Bereich. Ausgeliehene Decimal-/NullDecimal-Handles müssen beim Decodieren noch aktiv sein; die Konvertierung erzeugt unabhängige Werte ohne Eingabekonsum. Generisches Serde-Trait-Verhalten wird durch konkrete JSON-Adapter gezeigt.

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
