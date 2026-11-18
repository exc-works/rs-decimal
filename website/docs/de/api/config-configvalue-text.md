---
locale: de
source_revision: 9cc3411fb84577d5ea175fe7b4b5d7e0d13f86f3ba54a79c3a0728b721c31211
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue::Text"
---
# config::ConfigValue::Text

Konfigurationseingabe Text wählen.

```rust
Text (& 'a str)
```

## Parameter, Rückgabe und übliche Verwendung

Exakten Dezimaltext ausleihen. Äußerer Leerraum wird vom Dezimalkern verarbeitet; äußere SQL-artige Anführungszeichen werden nicht entfernt. Nur exakt leerer Text bedeutet nullable-Abwesenheit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-text" locale="de" />

## Grenzen im Browser

Die Features config und serde sind aktiviert. Quelltext/-bytes sind auf 8192 UTF-8-Bytes begrenzt; Rust prüft Dezimaltext und decodierte JSON-Zeichenfolgen vor dem Parsen, sodass Escapes die [Ressourcenrichtlinie](../resource-policy) nicht umgehen. 128-Bit-Ganzzahlen benötigen bigint im gültigen Bereich. Ausgeliehene Decimal-/NullDecimal-Handles müssen beim Decodieren noch aktiv sein; die Konvertierung erzeugt unabhängige Werte ohne Eingabekonsum. Generisches Serde-Trait-Verhalten wird durch konkrete JSON-Adapter gezeigt.

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
