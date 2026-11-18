---
locale: de
source_revision: 76c1141f3b7421518d2bc3a9f102aa654b735f07d57812e6208ffd1b784510f2
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue"
---
# config::ConfigValue

Explizit typisierte Konfigurationswerte decodieren.

```rust
pub enum ConfigValue< 'a >
```

## Parameter, Rückgabe und übliche Verwendung

Der typisierte Konfigurationsadapter unterstützt Text/Textbytes, exakten numerischen Text, vorzeichenbehaftete/vorzeichenlose 128-Bit-Ganzzahlen, Gleitkommawerte und ausgeliehene Decimal-/NullDecimal-Werte. Rust benötigt config. Nicht-nullable-Decodierung weist Null und ungültige Nullable ab, statt den Zahlenwert null zu erfinden. Vorhandene Decimal und gültige Nullable werden mit vollständiger Darstellung einschließlich nicht initialisierter Null geklont. Text/Number/Bytes parsen exakten Dezimaltext ohne Entfernen von SQL-Anführungszeichen. Gleitkommawerte enthalten bereits binäre Rundung; verwenden Sie Text oder bigint für exakte Quellziffern. Browservarianten gehören zu einem Lauf und leihen Handles/Puffer bis zum Decodieren aus.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue" locale="de" />

## Grenzen im Browser

Die Features config und serde sind aktiviert. Quelltext/-bytes sind auf 8192 UTF-8-Bytes begrenzt; Rust prüft Dezimaltext und decodierte JSON-Zeichenfolgen vor dem Parsen, sodass Escapes die [Ressourcenrichtlinie](../resource-policy) nicht umgehen. 128-Bit-Ganzzahlen benötigen bigint im gültigen Bereich. Ausgeliehene Decimal-/NullDecimal-Handles müssen beim Decodieren noch aktiv sein; die Konvertierung erzeugt unabhängige Werte ohne Eingabekonsum. Generisches Serde-Trait-Verhalten wird durch konkrete JSON-Adapter gezeigt.

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
