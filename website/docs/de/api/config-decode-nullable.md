---
locale: de
source_revision: 77287e6c4550f824e46a4556e0aaf001ccf92f4e78984f337df5b480f5f48874
translation_provenance: en-US-german-reviewed
api_id: "config::decode_nullable"
---
# config::decode_nullable

Einen optionalen Konfigurationswert decodieren.

```rust
fn decode_nullable (value : ConfigValue < '_ >) -> Result < NullDecimal , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

value: ConfigValue; liefert `Result<NullDecimal, DecimalError>`/ein neues nullable-Handle. Null, exakt leeres Text und leeres Bytes werden ungültig mit innerer nicht initialisierter Null. Text nur aus Leerraum und leeres Number scheitern weiterhin beim Dezimalparsen. Vorhandene Nullable klonen Gültigkeit und verborgene innere Daten auch bei Ungültigkeit; ein ausgeliehener Decimal wird gültig, ohne Initialisierung zu erzwingen. Andere Eingaben folgen decode_decimal und werden bei Erfolg gültig. Eingaben werden nicht konsumiert.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-decode-nullable" locale="de" />

## Grenzen im Browser

Die Features config und serde sind aktiviert. Quelltext/-bytes sind auf 8192 UTF-8-Bytes begrenzt; Rust prüft Dezimaltext und decodierte JSON-Zeichenfolgen vor dem Parsen, sodass Escapes die [Ressourcenrichtlinie](../resource-policy) nicht umgehen. 128-Bit-Ganzzahlen benötigen bigint im gültigen Bereich. Ausgeliehene Decimal-/NullDecimal-Handles müssen beim Decodieren noch aktiv sein; die Konvertierung erzeugt unabhängige Werte ohne Eingabekonsum. Generisches Serde-Trait-Verhalten wird durch konkrete JSON-Adapter gezeigt.

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
