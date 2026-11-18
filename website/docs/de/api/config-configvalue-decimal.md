---
locale: de
source_revision: b283786af672117d0c3017e513f97f5df201a379427acab824b383812aa4cf93
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue::Decimal"
---
# config::ConfigValue::Decimal

Konfigurationseingabe Decimal wählen.

```rust
Decimal (& 'a Decimal)
```

## Parameter, Rückgabe und übliche Verwendung

Einen vorhandenen Decimal ausleihen und beim Decodieren Koeffizient, Skala und Initialisierung klonen. Die Quelle bleibt nutzbar. Freigeben/Verschieben der Browserquelle vor dem Decodieren macht die ausgeliehene Variante ungültig.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-decimal" locale="de" />

## Grenzen im Browser

Die Features config und serde sind aktiviert. Quelltext/-bytes sind auf 8192 UTF-8-Bytes begrenzt; Rust prüft Dezimaltext und decodierte JSON-Zeichenfolgen vor dem Parsen, sodass Escapes die [Ressourcenrichtlinie](../resource-policy) nicht umgehen. 128-Bit-Ganzzahlen benötigen bigint im gültigen Bereich. Ausgeliehene Decimal-/NullDecimal-Handles müssen beim Decodieren noch aktiv sein; die Konvertierung erzeugt unabhängige Werte ohne Eingabekonsum. Generisches Serde-Trait-Verhalten wird durch konkrete JSON-Adapter gezeigt.

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
