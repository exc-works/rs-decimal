---
locale: de
source_revision: 52852655da8fa507f812b593c8e5ba40f3e2342d29c1bda28c254c4a87933fda
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::impl Serialize"
---
# NullDecimal::impl Serialize

Über den Rust-Serde-Trait serialisieren.

```rust
impl Serialize for NullDecimal
```

## Parameter, Rückgabe und übliche Verwendung

serializer: S mit Implementierung von Serializer; liefert S::Ok/S::Error. Der Kern serialisiert kanonische Dezimalzeichenfolgen oder null. Ungültige nullable-Werte werden als null serialisiert; gültige Werte mit innerer nicht initialisierter Null als Zeichenfolge "0". Nachgestellte Nachkommanullen werden entfernt. Der Browseradapter serialize() wählt serde_json und liefert vollständigen JSON-Text. Beliebige Rust-Serializer werden nicht bereitgestellt.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-nulldecimal-impl-serialize" locale="de" />

## Grenzen im Browser

Die Features config und serde sind aktiviert. Quelltext/-bytes sind auf 8192 UTF-8-Bytes begrenzt; Rust prüft Dezimaltext und decodierte JSON-Zeichenfolgen vor dem Parsen, sodass Escapes die [Ressourcenrichtlinie](../resource-policy) nicht umgehen. 128-Bit-Ganzzahlen benötigen bigint im gültigen Bereich. Ausgeliehene Decimal-/NullDecimal-Handles müssen beim Decodieren noch aktiv sein; die Konvertierung erzeugt unabhängige Werte ohne Eingabekonsum. Generisches Serde-Trait-Verhalten wird durch konkrete JSON-Adapter gezeigt.

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
