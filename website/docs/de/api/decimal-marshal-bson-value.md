---
locale: de
source_revision: 18a49744a56694725a1fe7d677be386e97a4c76f0669871a289c86113d20f3e2
translation_provenance: en-US-german-reviewed
api_id: "Decimal::marshal_bson_value"
---
# Decimal::marshal_bson_value

Ein BSON-Typbyte und eine Skalar-Payload codieren.

```rust
fn marshal_bson_value (& self) -> Result < (u8 , Vec < u8 >) , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert `Result<(u8, Vec<u8>), DecimalError>`/[number, Uint8Array]. Rust benötigt das im Browserbuild aktivierte Feature bson. Das Tupel enthält einen rohen BSON-Skalar, kein vollständiges BSON-Dokument. Initialisierte Werte werden als BSON-Zeichenfolgen (Typ 2), nicht Decimal128 codiert. Nicht initialisierte Null wird Typ 10 mit leerer Payload. Die Zeichenfolgen-Payload enthält eine Little-Endian-i32-Bytelänge einschließlich abschließendem NUL, danach kanonischen Dezimaltext ohne nachgestellte Nachkommanullen. Die zurückgegebenen Payload-Bytes sind eine unabhängige Kopie; Codieren ändert weder Initialisierung noch Gültigkeit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-marshal-bson-value" locale="de" />

## Grenzen im Browser

Die Eingabe muss Uint8Array und kind eine Ganzzahl zwischen 0 und 255 sein. Eingaben sind vor dem Kopieren nach Wasm auf 65536 Bytes begrenzt. Rust prüft binäre Skalen bis 1000 und tatsächliche Koeffizientengrößen bis 16384 Bits. BSON-Zeichenfolgen folgen der Textgrenze von 8192 Bytes; Decimal128-Exponentenexpansion wird ebenfalls [auf Ressourcen geprüft](../resource-policy). Diese Grenzen können enger als die der nativen Bibliothek sein. Prüfungen erfolgen vor Empfängeränderungen; Kern- und Ressourcenfehler bleiben getrennt, verschobene Handles sind nicht wiederverwendbar.

## Verwandte Methoden

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
