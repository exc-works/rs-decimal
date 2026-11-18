---
locale: de
source_revision: 0b8d31dd4724ab3db9f7852e985c43d4d28683f87ed08364b96e0b23430c8c45
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::unmarshal_json"
---
# NullDecimal::unmarshal_json

json-Eingabe in einen vorhandenen NullDecimal decodieren.

```rust
fn unmarshal_json (& mut self , bytes : & [u8]) -> Result < () , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

bytes: &[u8]/Uint8Array; ändert den Empfänger und liefert Result<(), DecimalError>/undefined. Rust benötigt json. Übergeben Sie ursprüngliche UTF-8-JSON-Bytes: Zahlen werden ohne Gleitkommaumweg validiert, Escapes in Zeichenfolgen vor exaktem Dezimalparsen aufgelöst. Wandeln Sie Zahlentoken nicht zuerst mit JSON.parse in JS-Zahlen um. JSON-Leerraum ist ausschließlich space/tab/CR/LF. JSON null löscht die Gültigkeit und setzt den inneren Decimal auf nicht initialisierte Null zurück. Erfolgreiches Parsen einer Eingabe außer null initialisiert die Dezimalzahl, erhält nachgestellte Skala bei Zahlenwerten ungleich null und normalisiert geparste Null auf Skala null. Setzt valid=true. Fehler erhalten den vollständigen Empfängerzustand. Bibliotheksdecoderfehler behalten Unmarshal und zugrunde liegende Arten; Ressourcenfehler treten ebenfalls vor einer Änderung auf.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-unmarshal-json" locale="de" />

## Grenzen im Browser

Decodereingaben sind auf 8192 Bytes begrenzt; Parameterzeichenfolgen werden ebenfalls vor dem Kopieren nach Wasm auf Länge geprüft. Rust prüft die [Dezimalressourcenrichtlinie](../resource-policy) erneut für Text und decodierte JSON-Zeichenfolgen; Escapes umgehen daher keine Exponenten-/Skalengrenzen. Diagnosen für ungültiges UTF-8 sowie JSON-/Dezimalsyntax stammen vom Kern. Eingabe- und Ressourcenfehler ändern Empfänger nicht teilweise; verschobene Wrapper können weder gelesen noch verändert werden.

## Verwandte Methoden

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
