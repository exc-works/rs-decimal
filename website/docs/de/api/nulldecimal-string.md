---
locale: de
source_revision: 753b858d717076ef46e98ae0c0b43697fb31b0651286396a37e7378758f67207
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::string"
---
# NullDecimal::string

Kanonischen nullable-Text lesen.

```rust
fn string (& self) -> String
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert String/string. Gültige Werte verwenden den kanonischen Text des inneren Decimal ohne nachgestellte Nachkommanullen. Ungültige Werte liefern unabhängig von verborgenen inneren Daten das Literal null. Dies unterscheidet sich von marshal_text, das bei ungültigen Werten leere Bytes liefert. Lesen ändert weder Gültigkeit noch Initialisierung.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-string" locale="de" />

## Grenzen im Browser

Decodereingaben sind auf 8192 Bytes begrenzt; Parameterzeichenfolgen werden ebenfalls vor dem Kopieren nach Wasm auf Länge geprüft. Rust prüft die [Dezimalressourcenrichtlinie](../resource-policy) erneut für Text und decodierte JSON-Zeichenfolgen; Escapes umgehen daher keine Exponenten-/Skalengrenzen. Diagnosen für ungültiges UTF-8 sowie JSON-/Dezimalsyntax stammen vom Kern. Eingabe- und Ressourcenfehler ändern Empfänger nicht teilweise; verschobene Wrapper können weder gelesen noch verändert werden.

## Verwandte Methoden

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
