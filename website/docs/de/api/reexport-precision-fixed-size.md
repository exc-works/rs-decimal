---
locale: de
source_revision: 40283a0b4501fabd3b307e9ac7d1a0dd292a797503fe33899ba506e5b43fdac8
translation_provenance: en-US-german-reviewed
api_id: "reexport::PRECISION_FIXED_SIZE"
---
# reexport::PRECISION_FIXED_SIZE

Die Bytelänge des binären Präzisionspräfixes lesen.

```rust
pub use binary::PRECISION_FIXED_SIZE as PRECISION_FIXED_SIZE
```

## Parameter, Rückgabe und übliche Verwendung

Eine Rust-usize-Konstante mit Wert 4: die Bytelänge des Big-Endian-Skalenpräfixes im Go-kompatiblen Binärformat. Sie ist weder Dezimalziffernzahl noch Präzisionsgrenze. Der Browser stellt eine schreibgeschützte number aus Rust bereit. Der Reexport an der Crate-Wurzel ist Rust-Importsyntax.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-reexport-precision-fixed-size" locale="de" />

## Grenzen im Browser

Die Eingabe muss Uint8Array und kind eine Ganzzahl zwischen 0 und 255 sein. Eingaben sind vor dem Kopieren nach Wasm auf 65536 Bytes begrenzt. Rust prüft binäre Skalen bis 1000 und tatsächliche Koeffizientengrößen bis 16384 Bits. BSON-Zeichenfolgen folgen der Textgrenze von 8192 Bytes; Decimal128-Exponentenexpansion wird ebenfalls [auf Ressourcen geprüft](../resource-policy). Diese Grenzen können enger als die der nativen Bibliothek sein. Prüfungen erfolgen vor Empfängeränderungen; Kern- und Ressourcenfehler bleiben getrennt, verschobene Handles sind nicht wiederverwendbar.

## Verwandte Methoden

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
