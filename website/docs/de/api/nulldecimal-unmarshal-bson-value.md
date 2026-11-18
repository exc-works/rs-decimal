---
locale: de
source_revision: a350b5bea887bf82e5e3ddafba6453f683a9f5862d842b3e641b0a441cbb3289
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::unmarshal_bson_value"
---
# NullDecimal::unmarshal_bson_value

Einen BSON-Skalar in einen vorhandenen Wert decodieren.

```rust
fn unmarshal_bson_value (& mut self , kind : u8 , data : & [u8]) -> Result < () , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

kind: u8/number, data: &[u8]/Uint8Array; ändert den Empfänger und liefert `Result<(), DecimalError>`/undefined. Rust benötigt bson. Unterstützt double (1), string (2), null (10), int32 (16), int64 (18) und Decimal128 (19); numerische Payloads verwenden BSON-Little-Endian-Bytes. Übergeben Sie int64 als rohe Bytes, um Genauigkeitsverlust durch JS number zu vermeiden. BSON null löscht valid und setzt den inneren Decimal auf nicht initialisierte Null zurück. Erfolgreiche nullable-Decodierung einer Eingabe außer null setzt valid=true. Nicht unterstützte Arten, abgeschnittene Daten und nicht endliche Zahlen scheitern ohne Teiländerung. Kompatibilitätsdetails: Zeichenfolgendecodierung überspringt das deklarierte letzte Byte ohne NUL-Prüfung; Bytes hinter der benötigten Payload werden ignoriert. Decimal128 wird vom Rust-Kern decodiert und behält unterstützte Dezimalgenauigkeit; marshal_bson_value gibt weiterhin eine Zeichenfolge aus.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-nulldecimal-unmarshal-bson-value" locale="de" />

## Grenzen im Browser

Die Eingabe muss Uint8Array und kind eine Ganzzahl zwischen 0 und 255 sein. Eingaben sind vor dem Kopieren nach Wasm auf 65536 Bytes begrenzt. Rust prüft binäre Skalen bis 1000 und tatsächliche Koeffizientengrößen bis 16384 Bits. BSON-Zeichenfolgen folgen der Textgrenze von 8192 Bytes; Decimal128-Exponentenexpansion wird ebenfalls [auf Ressourcen geprüft](../resource-policy). Diese Grenzen können enger als die der nativen Bibliothek sein. Prüfungen erfolgen vor Empfängeränderungen; Kern- und Ressourcenfehler bleiben getrennt, verschobene Handles sind nicht wiederverwendbar.

## Verwandte Methoden

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
