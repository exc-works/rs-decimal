---
locale: de
source_revision: 8ab6c44a0e4cf327b955869493409ea9bcd447a1bc1451590926853f00b0ca0b
translation_provenance: en-US-german-reviewed
api_id: "Decimal::unmarshal_json"
---
# Decimal::unmarshal_json

json-Eingabe in einen vorhandenen Decimal decodieren.

```rust
fn unmarshal_json (& mut self , bytes : & [u8]) -> Result < () , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

bytes: &[u8]/Uint8Array; ändert den Empfänger und liefert Result<(), DecimalError>/undefined. Rust benötigt json. Übergeben Sie ursprüngliche UTF-8-JSON-Bytes: Zahlen werden ohne Gleitkommaumweg validiert, Escapes in Zeichenfolgen vor exaktem Dezimalparsen aufgelöst. Wandeln Sie Zahlentoken nicht zuerst mit JSON.parse in JS-Zahlen um. JSON-Leerraum ist ausschließlich space/tab/CR/LF. JSON null lässt den gesamten vorhandenen Decimal einschließlich Initialisierung unverändert. Erfolgreiches Parsen einer Eingabe außer null initialisiert die Dezimalzahl, erhält nachgestellte Skala bei Zahlenwerten ungleich null und normalisiert geparste Null auf Skala null. Fehler erhalten den vollständigen Empfängerzustand. Bibliotheksdecoderfehler behalten Unmarshal und zugrunde liegende Arten; Ressourcenfehler treten ebenfalls vor einer Änderung auf.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-unmarshal-json" locale="de" />

## Grenzen im Browser

Decodereingaben sind auf 8192 Bytes begrenzt; Parameterzeichenfolgen werden ebenfalls vor dem Kopieren nach Wasm auf Länge geprüft. Rust prüft die [Dezimalressourcenrichtlinie](../resource-policy) erneut für Text und decodierte JSON-Zeichenfolgen; Escapes umgehen daher keine Exponenten-/Skalengrenzen. Diagnosen für ungültiges UTF-8 sowie JSON-/Dezimalsyntax stammen vom Kern. Eingabe- und Ressourcenfehler ändern Empfänger nicht teilweise; verschobene Wrapper können weder gelesen noch verändert werden.

## Verwandte Methoden

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
