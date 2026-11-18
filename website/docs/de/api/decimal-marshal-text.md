---
locale: de
source_revision: 38a30bb0b4a307093eafd823ad26d297985a36153b191d7c695a0f68960ea0e6
translation_provenance: en-US-german-reviewed
api_id: "Decimal::marshal_text"
---
# Decimal::marshal_text

Decimal als text-Bytes codieren.

```rust
fn marshal_text (& self) -> Vec < u8 >
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert `Vec<u8>`/Uint8Array mit kanonischem ASCII-Dezimaltext. Nachgestellte Nachkommanullen werden entfernt. Die Standardnull von Decimal gibt ASCII 0 aus. Die zurückgegebenen Bytes sind eine unabhängige Kopie; Lesen ändert keinen Zustand.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-marshal-text" locale="de" />

## Grenzen im Browser

Decodereingaben sind auf 8192 Bytes begrenzt; Parameterzeichenfolgen werden ebenfalls vor dem Kopieren nach Wasm auf Länge geprüft. Rust prüft die [Dezimalressourcenrichtlinie](../resource-policy) erneut für Text und decodierte JSON-Zeichenfolgen; Escapes umgehen daher keine Exponenten-/Skalengrenzen. Diagnosen für ungültiges UTF-8 sowie JSON-/Dezimalsyntax stammen vom Kern. Eingabe- und Ressourcenfehler ändern Empfänger nicht teilweise; verschobene Wrapper können weder gelesen noch verändert werden.

## Verwandte Methoden

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
