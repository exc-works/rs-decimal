---
locale: de
source_revision: 4748c543f00fa16cc184b2f3ac3218ce12c09982f49453b64f19d87393fb722a
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue::Nullable"
---
# config::ConfigValue::Nullable

Konfigurationseingabe Nullable wählen.

```rust
Nullable (& 'a NullDecimal)
```

## Parameter, Rückgabe und übliche Verwendung

Einen vorhandenen nullable-Wert ausleihen. decode_nullable klont Gültigkeit und verborgene Daten; decode_decimal akzeptiert nur gültige Werte und klont deren inneren Decimal. Änderungen vor dem Decodieren werden berücksichtigt.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-nullable" locale="de" />

## Grenzen im Browser

Die Features config und serde sind aktiviert. Quelltext/-bytes sind auf 8192 UTF-8-Bytes begrenzt; Rust prüft Dezimaltext und decodierte JSON-Zeichenfolgen vor dem Parsen, sodass Escapes die [Ressourcenrichtlinie](../resource-policy) nicht umgehen. 128-Bit-Ganzzahlen benötigen bigint im gültigen Bereich. Ausgeliehene Decimal-/NullDecimal-Handles müssen beim Decodieren noch aktiv sein; die Konvertierung erzeugt unabhängige Werte ohne Eingabekonsum. Generisches Serde-Trait-Verhalten wird durch konkrete JSON-Adapter gezeigt.

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
