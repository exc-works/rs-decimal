---
locale: de
source_revision: ce7a1e6d51ea25aa6e22728a52c66d3bfd202ca324cd22c0d1d0545a351bced2
translation_provenance: en-US-german-reviewed
api_id: "Decimal::marshal_json"
---
# Decimal::marshal_json

Decimal als json-Bytes codieren.

```rust
fn marshal_json (& self) -> Vec < u8 >
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert `Vec<u8>`/Uint8Array mit JSON-Bytes. Rust benötigt json; der Browserbuild aktiviert es. Initialisierte Decimal-Werte werden als kanonische Zeichenfolgen in Anführungszeichen codiert; nicht initialisierte Standardnull als JSON null. Die zurückgegebenen Bytes sind eine unabhängige Kopie; Lesen ändert keinen Zustand.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-marshal-json" locale="de" />

## Grenzen im Browser

Decodereingaben sind auf 8192 Bytes begrenzt; Parameterzeichenfolgen werden ebenfalls vor dem Kopieren nach Wasm auf Länge geprüft. Rust prüft die [Dezimalressourcenrichtlinie](../resource-policy) erneut für Text und decodierte JSON-Zeichenfolgen; Escapes umgehen daher keine Exponenten-/Skalengrenzen. Diagnosen für ungültiges UTF-8 sowie JSON-/Dezimalsyntax stammen vom Kern. Eingabe- und Ressourcenfehler ändern Empfänger nicht teilweise; verschobene Wrapper können weder gelesen noch verändert werden.

## Verwandte Methoden

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
