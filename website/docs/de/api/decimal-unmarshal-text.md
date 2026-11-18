---
locale: de
source_revision: 7e0df53b59ae6c40ced288a2a324fe9ef0557e3dc38c029cf8a66e1749c4ce28
translation_provenance: en-US-german-reviewed
api_id: "Decimal::unmarshal_text"
---
# Decimal::unmarshal_text

text-Eingabe in einen vorhandenen Decimal decodieren.

```rust
fn unmarshal_text (& mut self , text : & [u8]) -> Result < () , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

bytes: &[u8]/Uint8Array; ändert den Empfänger und liefert Result<(), DecimalError>/undefined. Parst Dezimaltext einschließlich entferntem äußerem Leerraum und Exponentensyntax. Leerer Text und das Literal null sind ungültiger Dezimaltext; verwenden Sie nullable-Decodierung, wenn Leere Abwesenheit bedeutet. Erfolgreiches Parsen einer Eingabe außer null initialisiert die Dezimalzahl, erhält nachgestellte Skala bei Zahlenwerten ungleich null und normalisiert geparste Null auf Skala null. Fehler erhalten den vollständigen Empfängerzustand. Bibliotheksdecoderfehler behalten Unmarshal und zugrunde liegende Arten; Ressourcenfehler treten ebenfalls vor einer Änderung auf.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-unmarshal-text" locale="de" />

## Grenzen im Browser

Decodereingaben sind auf 8192 Bytes begrenzt; Parameterzeichenfolgen werden ebenfalls vor dem Kopieren nach Wasm auf Länge geprüft. Rust prüft die [Dezimalressourcenrichtlinie](../resource-policy) erneut für Text und decodierte JSON-Zeichenfolgen; Escapes umgehen daher keine Exponenten-/Skalengrenzen. Diagnosen für ungültiges UTF-8 sowie JSON-/Dezimalsyntax stammen vom Kern. Eingabe- und Ressourcenfehler ändern Empfänger nicht teilweise; verschobene Wrapper können weder gelesen noch verändert werden.

## Verwandte Methoden

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
