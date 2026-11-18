---
locale: de
source_revision: 2c10c2e73488c11940444f40ef8c96bd7f288472b5195e9bdeb57320696a7e16
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue::Unsupported"
---
# config::ConfigValue::Unsupported

Konfigurationseingabe Unsupported wählen.

```rust
Unsupported
```

## Parameter, Rückgabe und übliche Verwendung

Einen nicht unterstützten Konfigurationstyp ohne automatische Umwandlung darstellen. Beide Decoder liefern Unmarshal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-unsupported" locale="de" />

## Grenzen im Browser

Die Features config und serde sind aktiviert. Quelltext/-bytes sind auf 8192 UTF-8-Bytes begrenzt; Rust prüft Dezimaltext und decodierte JSON-Zeichenfolgen vor dem Parsen, sodass Escapes die [Ressourcenrichtlinie](../resource-policy) nicht umgehen. 128-Bit-Ganzzahlen benötigen bigint im gültigen Bereich. Ausgeliehene Decimal-/NullDecimal-Handles müssen beim Decodieren noch aktiv sein; die Konvertierung erzeugt unabhängige Werte ohne Eingabekonsum. Generisches Serde-Trait-Verhalten wird durch konkrete JSON-Adapter gezeigt.

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
