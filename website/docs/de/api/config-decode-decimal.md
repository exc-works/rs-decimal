---
locale: de
source_revision: 9abf8678e26851a7c2e2283fba52cbf7f426bb86feeae64e6b8312a6a7254ba7
translation_provenance: en-US-german-reviewed
api_id: "config::decode_decimal"
---
# config::decode_decimal

Explizit typisierte Konfigurationswerte decodieren.

```rust
fn decode_decimal (value : ConfigValue < '_ >) -> Result < Decimal , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

value: ConfigValue; liefert `Result<Decimal, DecimalError>`/ein neues Decimal-Handle. Rust benötigt config. Nicht-nullable-Decodierung weist Null und ungültige Nullable ab, statt den Zahlenwert null zu erfinden. Vorhandene Decimal und gültige Nullable werden mit vollständiger Darstellung einschließlich nicht initialisierter Null geklont. Text/Number/Bytes parsen exakten Dezimaltext ohne Entfernen von SQL-Anführungszeichen. Gleitkommawerte enthalten bereits binäre Rundung; verwenden Sie Text oder bigint für exakte Quellziffern. Browservarianten gehören zu einem Lauf und leihen Handles/Puffer bis zum Decodieren aus.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-decode-decimal" locale="de" />

## Grenzen im Browser

Die Features config und serde sind aktiviert. Quelltext/-bytes sind auf 8192 UTF-8-Bytes begrenzt; Rust prüft Dezimaltext und decodierte JSON-Zeichenfolgen vor dem Parsen, sodass Escapes die [Ressourcenrichtlinie](../resource-policy) nicht umgehen. 128-Bit-Ganzzahlen benötigen bigint im gültigen Bereich. Ausgeliehene Decimal-/NullDecimal-Handles müssen beim Decodieren noch aktiv sein; die Konvertierung erzeugt unabhängige Werte ohne Eingabekonsum. Generisches Serde-Trait-Verhalten wird durch konkrete JSON-Adapter gezeigt.

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
