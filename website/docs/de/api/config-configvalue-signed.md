---
locale: de
source_revision: 5bb3ef8c2d718846f6ac7a4004c6720c37095a64615189ff33906f2fba479837
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue::Signed"
---
# config::ConfigValue::Signed

Konfigurationseingabe Signed wählen.

```rust
Signed (i128)
```

## Parameter, Rückgabe und übliche Verwendung

Ein vorzeichenbehaftetes i128 exakt transportieren. Der Browser verlangt bigint in [-2^127, 2^127-1]; JS-Zahlen und Werte außerhalb des Bereichs werden vor Konvertierung abgewiesen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-signed" locale="de" />

## Grenzen im Browser

Die Features config und serde sind aktiviert. Quelltext/-bytes sind auf 8192 UTF-8-Bytes begrenzt; Rust prüft Dezimaltext und decodierte JSON-Zeichenfolgen vor dem Parsen, sodass Escapes die [Ressourcenrichtlinie](../resource-policy) nicht umgehen. 128-Bit-Ganzzahlen benötigen bigint im gültigen Bereich. Ausgeliehene Decimal-/NullDecimal-Handles müssen beim Decodieren noch aktiv sein; die Konvertierung erzeugt unabhängige Werte ohne Eingabekonsum. Generisches Serde-Trait-Verhalten wird durch konkrete JSON-Adapter gezeigt.

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
