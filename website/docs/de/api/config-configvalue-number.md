---
locale: de
source_revision: 74e7f2ffd43f198c10e7ebd972920c487311d6c7ab45db8c6281425ea90f9c14
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue::Number"
---
# config::ConfigValue::Number

Konfigurationseingabe Number wählen.

```rust
Number (& 'a str)
```

## Parameter, Rückgabe und übliche Verwendung

Numerischen Quelltext ohne Gleitkommakonvertierung transportieren. Der Kern parst Dezimalsyntax; diese Variante validiert keine vollständige JSON-Grammatik. Leeres Number ist auch für nullable-Ziele ungültig.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-number" locale="de" />

## Grenzen im Browser

Die Features config und serde sind aktiviert. Quelltext/-bytes sind auf 8192 UTF-8-Bytes begrenzt; Rust prüft Dezimaltext und decodierte JSON-Zeichenfolgen vor dem Parsen, sodass Escapes die [Ressourcenrichtlinie](../resource-policy) nicht umgehen. 128-Bit-Ganzzahlen benötigen bigint im gültigen Bereich. Ausgeliehene Decimal-/NullDecimal-Handles müssen beim Decodieren noch aktiv sein; die Konvertierung erzeugt unabhängige Werte ohne Eingabekonsum. Generisches Serde-Trait-Verhalten wird durch konkrete JSON-Adapter gezeigt.

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
