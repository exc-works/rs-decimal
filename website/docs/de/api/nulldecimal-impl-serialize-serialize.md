---
locale: de
source_revision: 67d49a92b2e2ca62692f06791c5907e6136e1bf8593cdb08edcf4fb008700159
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::impl Serialize::serialize"
---
# NullDecimal::impl Serialize::serialize

Über den Rust-Serde-Trait serialisieren.

```rust
fn serialize < S : Serializer > (& self , s : S) -> Result < S :: Ok , S :: Error >
```

## Parameter, Rückgabe und übliche Verwendung

serializer: S mit Implementierung von Serializer; liefert S::Ok/S::Error. Der Kern serialisiert kanonische Dezimalzeichenfolgen oder null. Ungültige nullable-Werte werden als null serialisiert; gültige Werte mit innerer nicht initialisierter Null als Zeichenfolge "0". Nachgestellte Nachkommanullen werden entfernt. Der Browseradapter serialize() wählt serde_json und liefert vollständigen JSON-Text. Beliebige Rust-Serializer werden nicht bereitgestellt.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-nulldecimal-impl-serialize-serialize" locale="de" />

## Grenzen im Browser

Die Features config und serde sind aktiviert. Quelltext/-bytes sind auf 8192 UTF-8-Bytes begrenzt; Rust prüft Dezimaltext und decodierte JSON-Zeichenfolgen vor dem Parsen, sodass Escapes die [Ressourcenrichtlinie](../resource-policy) nicht umgehen. 128-Bit-Ganzzahlen benötigen bigint im gültigen Bereich. Ausgeliehene Decimal-/NullDecimal-Handles müssen beim Decodieren noch aktiv sein; die Konvertierung erzeugt unabhängige Werte ohne Eingabekonsum. Generisches Serde-Trait-Verhalten wird durch konkrete JSON-Adapter gezeigt.

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
