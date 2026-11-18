---
locale: de
source_revision: 178894cd0af1970f19442cd816169372cdb2496adc78c87aaec82dd961b0da2e
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue::impl Copy"
---
# config::ConfigValue::impl Copy

Die native Copy-Implementierung verwenden.

```rust
impl Copy for config::ConfigValue
```

## Parameter, Rückgabe und übliche Verwendung

ConfigValue ist ein kleines typisiertes Enum. Clone/Copy erhalten ausgeliehene Referenzen, statt deren Ziel tief zu kopieren. Debug ist Diagnosetext, kein stabiles Übertragungsformat. Native Rust-Trait-Syntax wird separat getestet und ist keine Browsermethode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-impl-copy" locale="de" />

## Grenzen im Browser

Die Features config und serde sind aktiviert. Quelltext/-bytes sind auf 8192 UTF-8-Bytes begrenzt; Rust prüft Dezimaltext und decodierte JSON-Zeichenfolgen vor dem Parsen, sodass Escapes die [Ressourcenrichtlinie](../resource-policy) nicht umgehen. 128-Bit-Ganzzahlen benötigen bigint im gültigen Bereich. Ausgeliehene Decimal-/NullDecimal-Handles müssen beim Decodieren noch aktiv sein; die Konvertierung erzeugt unabhängige Werte ohne Eingabekonsum. Generisches Serde-Trait-Verhalten wird durch konkrete JSON-Adapter gezeigt.

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
