---
locale: de
source_revision: a26edb51e2fbe0ae6bac5abd8ce3e12bd13315bb3e836a0d554d5f19af0430ce
translation_provenance: en-US-german-reviewed
api_id: "config::deserialize_json_decimal"
---
# config::deserialize_json_decimal

Ein exaktes JSON-Konfigurationsfeld deserialisieren.

```rust
fn deserialize_json_decimal < 'de , D : Deserializer < 'de > > (deserializer : D ,) -> Result < Decimal , D :: Error >
```

## Parameter, Rückgabe und übliche Verwendung

deserializer: D mit Implementierung von Serde Deserializer; vorgesehen für #[serde(deserialize_with = "rs_decimal::config::deserialize_json_decimal")]. Benötigt eine serde_json-Quelle und das Feature config. Erhält ursprüngliche Zahlentoken ohne Gleitkommaumweg. Der Browser stellt einen konkreten Adapter für eine vollständige JSON-Skalarquellzeichenfolge bereit, keinen beliebigen Deserializer. JSON null und leere Zeichenfolgen werden für das nicht-nullable-Ziel abgewiesen. Fehler haben den Serde-Deserializer-Fehlertyp; Browser-category="serde" erhält dessen Meldung, statt DecimalError-Arten zu erfinden. Nachgestellte Eingabe außer JSON-Leerraum wird abgewiesen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-deserialize-json-decimal" locale="de" />

## Grenzen im Browser

Die Features config und serde sind aktiviert. Quelltext/-bytes sind auf 8192 UTF-8-Bytes begrenzt; Rust prüft Dezimaltext und decodierte JSON-Zeichenfolgen vor dem Parsen, sodass Escapes die [Ressourcenrichtlinie](../resource-policy) nicht umgehen. 128-Bit-Ganzzahlen benötigen bigint im gültigen Bereich. Ausgeliehene Decimal-/NullDecimal-Handles müssen beim Decodieren noch aktiv sein; die Konvertierung erzeugt unabhängige Werte ohne Eingabekonsum. Generisches Serde-Trait-Verhalten wird durch konkrete JSON-Adapter gezeigt.

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
