---
locale: de
source_revision: ce07dfd04d96401a00570a69b98dc03abf2e70bda05348ea2e8e6bff0225c443
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::marshal_json"
---
# NullDecimal::marshal_json

NullDecimal als json-Bytes codieren.

```rust
fn marshal_json (& self) -> Vec < u8 >
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert `Vec<u8>`/Uint8Array mit JSON-Bytes. Rust benötigt json; der Browserbuild aktiviert es. Ungültige nullable-Werte werden als JSON null codiert; gültige Werte als kanonische Dezimalzeichenfolgen in Anführungszeichen, auch bei nicht initialisiertem innerem Decimal. Die zurückgegebenen Bytes sind eine unabhängige Kopie; Lesen ändert keinen Zustand.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-marshal-json" locale="de" />

## Grenzen im Browser

Decodereingaben sind auf 8192 Bytes begrenzt; Parameterzeichenfolgen werden ebenfalls vor dem Kopieren nach Wasm auf Länge geprüft. Rust prüft die [Dezimalressourcenrichtlinie](../resource-policy) erneut für Text und decodierte JSON-Zeichenfolgen; Escapes umgehen daher keine Exponenten-/Skalengrenzen. Diagnosen für ungültiges UTF-8 sowie JSON-/Dezimalsyntax stammen vom Kern. Eingabe- und Ressourcenfehler ändern Empfänger nicht teilweise; verschobene Wrapper können weder gelesen noch verändert werden.

## Verwandte Methoden

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
