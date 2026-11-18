---
locale: de
source_revision: 76e0ede0f8f2ecf84ebf0544bf21902acebb1483558125958da665053c84f11f
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue::impl Debug"
---
# config::ConfigValue::impl Debug

Die native Debug-Implementierung verwenden.

```rust
impl Debug for config::ConfigValue
```

## Parameter, Rückgabe und übliche Verwendung

ConfigValue ist ein kleines typisiertes Enum. Clone/Copy erhalten ausgeliehene Referenzen, statt deren Ziel tief zu kopieren. Debug ist Diagnosetext, kein stabiles Übertragungsformat. Native Rust-Trait-Syntax wird separat getestet und ist keine Browsermethode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-impl-debug" locale="de" />

## Grenzen im Browser

Die Features config und serde sind aktiviert. Quelltext/-bytes sind auf 8192 UTF-8-Bytes begrenzt; Rust prüft Dezimaltext und decodierte JSON-Zeichenfolgen vor dem Parsen, sodass Escapes die [Ressourcenrichtlinie](../resource-policy) nicht umgehen. 128-Bit-Ganzzahlen benötigen bigint im gültigen Bereich. Ausgeliehene Decimal-/NullDecimal-Handles müssen beim Decodieren noch aktiv sein; die Konvertierung erzeugt unabhängige Werte ohne Eingabekonsum. Generisches Serde-Trait-Verhalten wird durch konkrete JSON-Adapter gezeigt.

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
