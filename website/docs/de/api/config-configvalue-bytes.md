---
locale: de
source_revision: 3576d2b52e6de01c1594ed23748d6a482f290fdb6f5b45fc15ec07a4cf297d34
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue::Bytes"
---
# config::ConfigValue::Bytes

Konfigurationseingabe Bytes wählen.

```rust
Bytes (& 'a [u8])
```

## Parameter, Rückgabe und übliche Verwendung

Bytes mit Dezimaltext ausleihen, keine binäre Dezimal-Payload. Änderungen am Browser-Uint8Array vor dem Decodieren werden berücksichtigt; Decodieren verändert das Array nie. Ungültiges UTF-8 scheitert beim Dezimalparsen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-bytes" locale="de" />

## Grenzen im Browser

Die Features config und serde sind aktiviert. Quelltext/-bytes sind auf 8192 UTF-8-Bytes begrenzt; Rust prüft Dezimaltext und decodierte JSON-Zeichenfolgen vor dem Parsen, sodass Escapes die [Ressourcenrichtlinie](../resource-policy) nicht umgehen. 128-Bit-Ganzzahlen benötigen bigint im gültigen Bereich. Ausgeliehene Decimal-/NullDecimal-Handles müssen beim Decodieren noch aktiv sein; die Konvertierung erzeugt unabhängige Werte ohne Eingabekonsum. Generisches Serde-Trait-Verhalten wird durch konkrete JSON-Adapter gezeigt.

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
