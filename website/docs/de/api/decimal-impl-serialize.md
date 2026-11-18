---
locale: de
source_revision: fa036d4ed1d96137decee6ee5f67e9b5d0d5b6d6fed77430c428472e7a62d1ff
translation_provenance: en-US-german-reviewed
api_id: "Decimal::impl Serialize"
---
# Decimal::impl Serialize

Über den Rust-Serde-Trait serialisieren.

```rust
impl Serialize for Decimal
```

## Parameter, Rückgabe und übliche Verwendung

serializer: S mit Implementierung von Serializer; liefert S::Ok/S::Error. Der Kern serialisiert kanonische Dezimalzeichenfolgen oder null. Nicht initialisierter Decimal wird als null serialisiert; initialisierte Null als Zeichenfolge "0". Nachgestellte Nachkommanullen werden entfernt. Der Browseradapter serialize() wählt serde_json und liefert vollständigen JSON-Text. Beliebige Rust-Serializer werden nicht bereitgestellt.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-decimal-impl-serialize" locale="de" />

## Grenzen im Browser

Die Features config und serde sind aktiviert. Quelltext/-bytes sind auf 8192 UTF-8-Bytes begrenzt; Rust prüft Dezimaltext und decodierte JSON-Zeichenfolgen vor dem Parsen, sodass Escapes die [Ressourcenrichtlinie](../resource-policy) nicht umgehen. 128-Bit-Ganzzahlen benötigen bigint im gültigen Bereich. Ausgeliehene Decimal-/NullDecimal-Handles müssen beim Decodieren noch aktiv sein; die Konvertierung erzeugt unabhängige Werte ohne Eingabekonsum. Generisches Serde-Trait-Verhalten wird durch konkrete JSON-Adapter gezeigt.

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
